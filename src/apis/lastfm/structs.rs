use std::str::FromStr;

pub enum TimePeriod {
    Week,
    Month,
    Quarter,
    HalfYear,
    Year,
    Alltime,
}

impl TimePeriod {
    pub fn apiformat(&self) -> &str {
        match *self {
            TimePeriod::Week => "7day",
            TimePeriod::Month => "1month",
            TimePeriod::Quarter => "3month",
            TimePeriod::HalfYear => "6month",
            TimePeriod::Year => "12month",
            TimePeriod::Alltime => "overall",
        }
    }
}

impl FromStr for TimePeriod {
    type Err = ();

    fn from_str(s: &str) -> Result<TimePeriod, ()> {
        match s {
            "week" | "7days" | "7day" => Ok(TimePeriod::Week),
            "month" | "1month" | "1months" => Ok(TimePeriod::Month),
            "3months" | "3month" => Ok(TimePeriod::Quarter),
            "6months" | "6month" | "halfyear" => Ok(TimePeriod::HalfYear),
            "year" | "12months" | "12month" => Ok(TimePeriod::Year),
            "overall" | "alltime" | "all" => Ok(TimePeriod::Alltime),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastFmError {
    pub error: u8,
    pub message: String,
}

// api return types

#[derive(Deserialize, Debug)]
pub struct GetInfo {
    pub user: User,
}

use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Debug)]
pub struct GetTopAlbums {
    pub topalbums: TopAlbums,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    #[serde(rename = "#text")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct UnixTime {
    #[serde(rename = "#text")]
    pub timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct User {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub playcount: u32,
    pub name: String,
    pub url: String,
    pub country: String,
    pub image: Vec<Image>,
    pub registered: UnixTime,
    pub realname: String,
}

#[derive(Deserialize, Debug)]
pub struct TopAlbums {
    #[serde(rename = "album")]
    pub albums: Vec<Album>,
    #[serde(rename = "@attr")]
    pub attr: PaginationAttr,
}

#[derive(Deserialize, Debug)]
pub struct AlbumArtist {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub artist: AlbumArtist,
    pub image: Vec<Image>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub playcount: u32,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PaginationAttr {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub page: u32,
    #[serde(rename = "perPage")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub per_page: u32,
    pub user: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total: u32,
    #[serde(rename = "totalPages")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_pages: u32,
}
