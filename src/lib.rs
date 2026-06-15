use serde::{Deserialize, Serialize};

pub mod api_fetcher;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NowPlaying {
    is_playing: bool,
    progress_ms: Option<u64>,
    item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    name: String,
    duration_ms: Option<u64>,
    artists: Vec<Artist>,
    album: Album,
    preview_url: Option<String>,
    available_markets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    name: String,
    images: Vec<Image>,
    available_markets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
}
