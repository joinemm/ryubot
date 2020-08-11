use log::error;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

mod modules;
use modules::database;
use modules::events;
use modules::types::*;

mod commands;
use commands::{general::*, owner::*};

#[group]
#[commands(ping, joke)]
struct General;

#[group]
#[commands(quit, dbe, dbq)]
struct Owner;

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("No token found!");
    let database_credentials = env::var("DATABASE_CREDENTIALS").expect("No database credentials found!");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let reqwest_client = reqwest::Client::new();
    let pool = database::get_pool(&database_credentials).await.unwrap();

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .prefix("=")
                .on_mention(Some(bot_id))
                .case_insensitivity(true)
        })
        .after(events::after_hook)
        .before(events::before_hook)
        .on_dispatch_error(events::dispatch_error)
        .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP)
        .help(&HELP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(events::Handler)
        .await
        .expect("Error creating client!");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestClient>(Arc::new(reqwest_client));
        data.insert::<ConnectionPool>(pool.clone());
    }

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
