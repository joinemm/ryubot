use serenity::{
    client::bridge::gateway::GatewayIntents, framework::StandardFramework, http::Http, prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};
use tracing::{error, instrument};
use tracing_subscriber;
#[macro_use]
extern crate quick_error;
mod modules;
use modules::database;
use modules::events;
use modules::types::*;

mod apis;
use apis::lastfm;

mod commands;
use commands::{general::*, lastfm::*, owner::*};

#[tokio::main]
#[instrument]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("No token found!");
    let database_credentials = env::var("DATABASE_URL").expect("No database credentials found!");
    let lastfm_api_key = env::var("LASTFM_TOKEN").expect("No lastfm api key found!");
    let lastfm_secret = env::var("LASTFM_SECRET").expect("No lastfm secret found!");

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

    let reqwest_client = Arc::new(reqwest::Client::new());
    let pool = database::get_pool(&database_credentials).await.unwrap();
    let lastfm_client = lastfm::api::LastFm::new(lastfm_api_key, lastfm_secret);

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
        .group(&LASTFM_GROUP)
        .group(&OWNER_GROUP)
        .help(&HELP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(events::Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client!");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestClient>(reqwest_client);
        data.insert::<ConnectionPool>(pool.clone());
        data.insert::<LastFmClient>(lastfm_client);
    }

    if let Err(why) = client.start_autosharded().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
