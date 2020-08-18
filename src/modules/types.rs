use crate::apis::lastfm;
use reqwest::Client as Reqwest;
use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use sqlx::PgPool;
use std::sync::Arc;
use std::time;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Reqwest>;
}

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub struct LastFmClient;

impl TypeMapKey for LastFmClient {
    type Value = lastfm::api::LastFm;
}
