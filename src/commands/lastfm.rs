use crate::apis::lastfm::structs::*;
use crate::modules::parsers;
use crate::modules::types::*;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::*,
};
use std::cmp::max;

use crate::apis::lastfm;

#[group]
#[prefix = "fm"]
#[commands(profile, topalbums, ping)]
struct LastFm;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let lastfm_client = data.get::<LastFmClient>().unwrap();
    let bot_msg = match lastfm_client.ping(reqwest_client).await {
        Ok(ms) => format!("Current Last.FM API delay is `{:?}`", ms),
        Err(e) => format!(
            ":warning: Unable to connect to the Last.FM API\n```rs\n{}\n```",
            e
        ),
    };

    msg.channel_id.say(&ctx, bot_msg).await?;
    Ok(())
}

#[command]
async fn profile(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let username = args.single::<String>()?;
    let body: GetInfo = lastfm::user::get_info(&ctx, &username).await?;

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(body.user.name)
                    .field("Profile", body.user.url, true)
                    .thumbnail(&body.user.image.last().unwrap().url)
            });
            m
        })
        .await?;

    Ok(())
}

#[command]
async fn topalbums(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let username = args.single::<String>().unwrap();
    let period = parsers::optional_argument(&mut args, TimePeriod::Alltime);
    let limit = parsers::optional_argument(&mut args, 10);

    let body: GetTopAlbums = lastfm::user::get_top_albums(
        &ctx,
        &username,
        &[("limit", &limit.to_string()), ("period", period.apiformat())],
    )
    .await?;

    if body.topalbums.albums.is_empty() {
        msg.channel_id.say(&ctx, "There are no albums to show!").await?;
        return Ok(());
    }

    let mut rows = vec![];

    for i in 0..max(limit, body.topalbums.albums.len()) {
        let album = &body.topalbums.albums[i];
        rows.push(format!("{} plays : {}", album.playcount, album.name))
    }

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("{}'s top albums", body.topalbums.attr.user))
                    .thumbnail(&body.topalbums.albums[0].image.last().unwrap().url)
                    .description(rows.join("\n"))
            });
            m
        })
        .await?;

    Ok(())
}
