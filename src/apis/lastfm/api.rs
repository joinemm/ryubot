use super::structs::*;
use std::collections::HashMap;
extern crate reqwest;
use md5;
use serde::de::DeserializeOwned;
use serde_json;
use std::sync::Arc;
use std::time::{Duration, Instant};

type ParamsMap = HashMap<String, String>;

pub struct LastFm {
    api_key: String,
    secret: String,
    base_url: &'static str,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// An error occurred while parsing the received JSON
        ParsingError(err: serde_json::Error) {
            display("Parsing Error: {:?}", err)
        }

        /// An error returned by the Lastfm API
         LastFmError(err: LastFmError) {
             display("Error code {}: {}", err.error, err.message)
         }

         /// An error in the HTTP request
         HttpError(err: reqwest::Error) {
             display("HTTP Error {:?}", err)
         }
    }
}

type LastFmResult<T> = Result<T, Error>;

impl LastFm {
    pub fn new(api_key: String, secret: String) -> Self {
        Self {
            api_key,
            secret,
            base_url: "https://ws.audioscrobbler.com/2.0/",
        }
    }

    pub async fn request<T>(
        &self,
        reqwest_client: &Arc<reqwest::Client>,
        method: String,
        mut params: ParamsMap,
    ) -> LastFmResult<T>
    where
        T: DeserializeOwned,
    {
        params.insert("method".to_string(), method);
        params.insert("api_key".to_string(), self.api_key.to_string());
        if params.contains_key("sk") {
            let signature = self.sign_call(&params);
            params.insert("api_sig".to_string(), signature);
        }
        let response_result = reqwest_client
            .get(self.base_url)
            .query(&params)
            .query(&[("format", "json")])
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

    fn sign_call(&self, params: &ParamsMap) -> String {
        let mut signature_string = String::new();
        for param in params {
            signature_string.push_str(param.0);
            signature_string.push_str(param.1);
        }

        signature_string.push_str(&self.secret.as_str());

        let signature = md5::compute(signature_string);
        return format!("{:x}", signature);
    }
}

fn deserialize<T>(response: String) -> LastFmResult<T>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<T>(&response) {
        Ok(body) => Ok(body),
        Err(_) => match serde_json::from_str::<LastFmError>(&response) {
            Ok(lastfm_error) => Err(Error::LastFmError(lastfm_error.clone())),
            Err(e) => Err(Error::ParsingError(e)),
        },
    }
}
