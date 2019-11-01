use super::stream::*;
use chrono::{DateTime, Utc};
use serde::de::{Deserialize, Deserializer};
use serde_derive::*;

// Type can be represented as:
//#[derive(Deserialize, Serialize, Debug)]
//#[serde(rename_all = "camelCase")]
//pub enum Preset { Movie, Series, Channel, Tv }
//#[derive(Deserialize, Serialize, Debug)]
//#[serde(untagged)]
//pub enum ItemType {
//    Preset(Preset),
//    Other(String)
//}
// or, some day it may be done with 1 enum:
// https://users.rust-lang.org/t/catchall-variant-in-serde/20748
// https://github.com/serde-rs/serde/pull/1382

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum PosterShape {
    Poster,
    Square,
    Landscape,
    #[serde(other)]
    Unspecified,
}
impl Default for PosterShape {
    fn default() -> Self {
        PosterShape::Unspecified
    }
}
impl PosterShape {
    pub fn is_unspecified(&self) -> bool {
        *self == PosterShape::Unspecified
    }
    // @TODO: auto-derive this?
    pub fn to_str(&self) -> &'static str {
        match self {
            PosterShape::Poster => "poster",
            PosterShape::Square => "square",
            PosterShape::Landscape => "landscape",
            PosterShape::Unspecified => "poster",
        }
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MetaPreview {
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub name: String,
    pub poster: Option<String>,
    pub logo: Option<String>,
    pub description: Option<String>,
    pub release_info: Option<String>,
    pub runtime: Option<String>,
    pub released: Option<DateTime<Utc>>,
    #[deprecated]
    #[serde(default)]
    pub genres: Vec<String>,
    #[deprecated]
    #[serde(default)]
    #[serde(alias = "director")]
    #[serde(deserialize_with = "deserialize_default_vec_string")]
    pub directors: Vec<String>,
    #[serde(default, skip_serializing_if = "PosterShape::is_unspecified")]
    pub poster_shape: PosterShape,
    pub trailer: Option<Stream>,
}

// https://github.com/Stremio/stremio-addon-sdk/blob/master/docs/api/responses/meta.md#meta-object
#[derive(PartialEq, Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDetail {
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub name: String,
    pub poster: Option<String>,
    pub background: Option<String>,
    pub logo: Option<String>,
    pub popularity: Option<f64>,
    pub description: Option<String>,
    pub release_info: Option<String>,
    pub runtime: Option<String>,
    pub released: Option<DateTime<Utc>>,
    #[deprecated]
    #[serde(default)]
    pub genres: Vec<String>,
    #[deprecated]
    #[serde(default)]
    #[serde(alias = "director")]
    #[serde(deserialize_with = "deserialize_default_vec_string")]
    pub directors: Vec<String>,
    #[deprecated]
    #[serde(default)]
    #[serde(alias = "writer")]
    pub writers: Vec<String>,
    #[deprecated]
    #[serde(default)]
    pub cast: Vec<String>,
    pub imdb_rating: Option<String>,
    #[serde(rename = "imdb_id")]
    pub imdb_id: Option<String>,
    #[serde(default, skip_serializing_if = "PosterShape::is_unspecified")]
    pub poster_shape: PosterShape,
    // @TODO: default to one video
    #[serde(default)]
    pub videos: Vec<Video>,
    // This is a video id; the case of the video not being in .videos must be handled at runtime
    pub featured_vid: Option<String>,
    // @TODO: decide between HashMap or a Vec(String, String)
    // @TODO: consider using a URL type
    // NOTE: this is also used instead of "website"
    #[serde(default)]
    pub external_urls: Vec<(String, String)>,
    // @TODO use some ISO language type
    //pub language: Option<String>,
    pub trailer: Option<Stream>,
}

// https://github.com/Stremio/stremio-addon-sdk/blob/master/docs/api/responses/meta.md#video-object
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    #[serde(alias = "name")]
    pub title: String,
    pub released: DateTime<Utc>,
    pub overview: Option<String>,
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub streams: Vec<Stream>,
    // @TODO: season AND episode (but they have to go together)
    #[serde(flatten)]
    pub series_info: Option<SeriesInfo>,
    pub trailer: Option<Stream>,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct SeriesInfo {
    pub season: u32,
    pub episode: u32,
}

fn deserialize_default_vec_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)
        .ok()
        .unwrap_or(Option::None)
        .unwrap_or(Vec::new()))
}
