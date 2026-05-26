use crate::NowPlaying;

pub async fn get_current_playing(
    client: &reqwest::Client,
    token: &str,
) -> Result<Option<NowPlaying>, reqwest::Error> {
    let url = "https://api.spotify.com/v1/me/player/currently-playing";
    let response = client.get(url).bearer_auth(token).send().await?;

    if response.status().as_u16() == 204 {
        Ok(None)
    } else if response.status().as_u16() == 200 {
        let track = response.json::<NowPlaying>().await?;
        return Ok(Some(track));
    } else {
        eprintln!("Request failed: {}", response.status());
        Ok(None)
    }
}
