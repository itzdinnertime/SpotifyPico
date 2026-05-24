#[derive(Serialize, Deserialize)]
struct NowPlaying {
    is_playing: bool,
    progress_ms: Option<u64>,
    item: Item,
}

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    duration_ms: Option<u64>,
    artists: Vec<Artist>,
    album: Album,
}

#[derive(Serialize, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Album {
    name: String,
    images: Vec<Image>,
}
