use super::api;
use super::structs::*;
extern crate serde_aux;
use crate::modules::types::*;
use serenity::prelude::*;

type LastFmResult<T> = Result<T, api::Error>;

pub async fn get_info(ctx: &Context, user: &str) -> LastFmResult<GetInfo> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let lastfm_client = data.get::<LastFmClient>().unwrap();
    lastfm_client
        .request::<GetInfo>(
            reqwest_client,
            (&[("method", "user.getInfo"), ("user", user)], &[]),
        )
        .await
}

pub async fn get_top_albums(
    ctx: &Context,
    user: &str,
    optional_params: &[(&str, &str)],
) -> LastFmResult<GetTopAlbums> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let lastfm_client = data.get::<LastFmClient>().unwrap();
    lastfm_client
        .request::<GetTopAlbums>(
            reqwest_client,
            (
                &[("method", "user.getTopAlbums"), ("user", user)],
                optional_params,
            ),
        )
        .await
}
