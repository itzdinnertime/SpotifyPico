use serde::{Deserialize, Serialize};

pub mod api_fetcher;

#[derive(Serialize, Deserialize, Debug)]
pub struct NowPlaying {
    is_playing: bool,
    progress_ms: Option<u64>,
    item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    duration_ms: Option<u64>,
    artists: Artist,
    album: Album,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    name: String,
    images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    url: String,
}
