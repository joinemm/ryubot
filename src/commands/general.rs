use super::super::modules::types::*;
use chrono::Utc;
use serenity::{
    framework::standard::{
        help_commands,
        macros::{command, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};
use std::collections::HashSet;

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[command]
#[description = "Get the current latency."]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Utc::now();
    let mut message = msg.channel_id.say(&ctx, "🏓").await?;
    let end = Utc::now();

    let round_trip = end - start;

    let data = ctx.data.read().await;
    let mut latencies = Vec::<String>::new();

    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let shard_manager = data.get::<ShardManagerContainer>().unwrap();
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    // Shards are backed by a "shard runner" responsible for processing events over the shard
    for (id, runner) in runners.iter() {
        latencies.push(match runner.latency {
            Some(latency) => format!(
                "Shard {} is {} with a latency of `{}`ms",
                id,
                runner.stage,
                latency.as_millis(),
            ),
            None => format!("Shard {} is {} and waiting for heartbeat...", id, runner.stage),
        });
    }
    let _ = message
        .edit(&ctx.http, |m| {
            m.embed(|e| {
                e.description(format!(
                    "Command roundtrip: `{}` ms\n{}",
                    round_trip.num_milliseconds(),
                    latencies.join("\n")
                ));
                e
            });
            m.content("");
            m
        })
        .await?;

    Ok(())
}

#[command]
#[min_args(1)]
#[usage = "< programming | miscellaneous | dark | pun >"]
async fn joke(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();

    let category = args.single::<String>()?.to_lowercase();
    let categories = ["programming", "miscellaneous", "dark", "pun"];
    if !categories.contains(&&*category) {
        msg.channel_id
            .say(
                &ctx,
                format!(
                    "Invalid category `{}`\nAvailable categories: `[ {} ]`",
                    category,
                    categories.join(" | ")
                ),
            )
            .await?;
        return Ok(());
    }
    let base_url = format!("https://sv443.net/jokeapi/v2/joke/{}", category);
    let url = reqwest::Url::parse_with_params(&base_url, &[("format", "txt")])?;
    let response = reqwest_client.get(url).send().await?;
    let content = response.text().await?;

    msg.channel_id.say(&ctx, content).await?;

    Ok(())
}
