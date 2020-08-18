use super::structs::*;
use super::user;
use crate::modules::types::*;
extern crate reqwest;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json;
use serenity::framework::standard::Args;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct LastFm {
    api_key: String,
    base_url: &'static str,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// An error occurred while parsing the received JSON
        ParsingError(err: serde_json::Error) {
            display("Parsing Error: {:?}", err)
        }

        /// An error returned by the APIs
         LastFmError(err: LastFmError) {
             display("Error code {}: {}", err.error, err.message)
         }

         HttpError(err: reqwest::Error) {
             display("HTTP Error {:?}", err)
         }
    }
}

type LastFmResult<T> = Result<T, Error>;

impl LastFm {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: api_key,
            base_url: "https://ws.audioscrobbler.com/2.0/",
        }
    }

    pub async fn request<T>(
        &self,
        reqwest_client: &Arc<reqwest::Client>,
        params: (&[(&str, &str)], &[(&str, &str)]),
    ) -> LastFmResult<T>
    where
        T: DeserializeOwned,
    {
        let response_result = reqwest_client
            .get(self.base_url)
            .query(&[("api_key", &self.api_key), ("format", &"json".to_string())])
            .query(params.0)
            .query(params.1)
            .send()
            .await;

        let response = match response_result {
            Ok(value) => value,
            Err(e) => return Err(Error::HttpError(e)),
        };

        println!("{}", response.url().as_str());
        let response_string = response.text().await.unwrap();
        deserialize(response_string)
    }

    pub async fn ping(&self, reqwest_client: &Arc<reqwest::Client>) -> Result<Duration, Error> {
        let start = Instant::now();
        let response_result = reqwest_client
            .get(self.base_url)
            .query(&[("api_key", &self.api_key), ("format", &"json".to_string())])
            .send()
            .await;

        let response = match response_result {
            Ok(value) => value,
            Err(e) => return Err(Error::HttpError(e)),
        };

        println!("{}", response.url().as_str());
        let response_string = response.text().await.unwrap();
        match deserialize::<LastFmError>(response_string) {
            Ok(_) => Ok(start.elapsed()),
            Err(e) => Err(e),
        }
    }
}

fn deserialize<T>(response: String) -> LastFmResult<T>
where
    T: DeserializeOwned,
{
    // match serde_json::from_str::<LastFmError>(&response) {
    //     Ok(lastfm_error) => Err(Error::LastFmError(lastfm_error.clone())),
    //     Err(_) => match serde_json::from_str::<T>(&response) {
    //         Ok(body) => Ok(body),
    //         Err(e) => Err(Error::ParsingError(e)),
    //     },
    // }

    match serde_json::from_str::<T>(&response) {
        Ok(body) => Ok(body),
        Err(_) => match serde_json::from_str::<LastFmError>(&response) {
            Ok(lastfm_error) => Err(Error::LastFmError(lastfm_error.clone())),
            Err(e) => Err(Error::ParsingError(e)),
        },
    }
}
