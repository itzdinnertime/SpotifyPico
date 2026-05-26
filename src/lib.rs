use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NowPlaying {
    is_playing: bool,
    progress_ms: Option<u64>,
    item: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    duration_ms: Option<u64>,
    artists: Artist,
    album: Album,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Album {
    name: String,
    images: Vec<Image>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    url: String,
}
