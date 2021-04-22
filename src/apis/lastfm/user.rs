use super::api;
use super::structs::*;
extern crate serde_aux;
use crate::modules::types::*;
use serenity::prelude::*;
use std::collections::HashMap;

type LastFmResult<T> = Result<T, api::Error>;
type ParamsMap = HashMap<String, String>;

fn param_with_auth(user_auth: UserAuthentication, optional_params: &[(&str, &str)]) -> ParamsMap {
    let mut params = ParamsMap::new();
    let auth_param = user_auth.params();
    params.insert(auth_param.0, auth_param.1);

    for (k, v) in optional_params {
        params.insert(k.to_string(), v.to_string());
    }

    params
}

pub async fn get_info(ctx: &Context, user_auth: UserAuthentication) -> LastFmResult<GetInfo> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let lastfm_client = data.get::<LastFmClient>().unwrap();
    lastfm_client
        .request::<GetInfo>(
            reqwest_client,
            "user.getInfo".to_string(),
            param_with_auth(user_auth, &[]),
        )
        .await
}

pub async fn get_top_albums(
    ctx: &Context,
    user_auth: UserAuthentication,
    optional_params: &[(&str, &str)],
) -> LastFmResult<GetTopAlbums> {
    let data = ctx.data.read().await;
    let reqwest_client = data.get::<ReqwestClient>().unwrap();
    let lastfm_client = data.get::<LastFmClient>().unwrap();
    lastfm_client
        .request::<GetTopAlbums>(
            reqwest_client,
            "user.getTopAlbums".to_string(),
            param_with_auth(user_auth, optional_params),
        )
        .await
}
