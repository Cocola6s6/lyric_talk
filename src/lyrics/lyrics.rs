use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Lyrics {
    pub response: ResponseData,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub hits: Vec<Hit>,
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub result: ResultData,
}

#[derive(Deserialize, Debug)]
pub struct ResultData {
    pub title: String,
    pub primary_artist: Artist,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,
}

impl Lyrics {
    // 获取歌词
    pub async fn fetch_lyrics(client: &Client, api_key: &str, song_title: &str, artist_name: &str)
                              -> Result<String, reqwest::Error> {
        let search_url = format!(
            "https://api.genius.com/search?q={}%20{}",
            song_title, artist_name
        );

        let auth_header = format!("Bearer {}", api_key);
        let res = client
            .get(&search_url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        let body = res.json::<Lyrics>().await?;
        if let Some(hit) = body.response.hits.get(0) {
            let result = &hit.result;
            let lyrics = Lyrics::get_lyrics_from_url(&result.url).await?;
            Ok(lyrics)
        } else {
            Ok("No lyrics found".to_string())
        }
    }

    // 从返回的html爬取歌词
    async fn get_lyrics_from_url(url: &str) -> Result<String, reqwest::Error> {
        let res = reqwest::get(url).await?;
        let body = res.text().await?;

        let document = Html::parse_document(&body);
        let lyrics_selector = Selector::parse(".Lyrics-sc-1bcc94c6-1.bzTABU").unwrap();
        if let Some(lyrics) = document.select(&lyrics_selector).next() {
            Ok(lyrics.text().collect::<Vec<_>>().join("\n"))
        } else {
            Ok("Lyrics not found".to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;
    use tokio;

    #[tokio::test]
    async fn test_fetch_lyrics() {
        let client = Client::new();
        let api_key = "V9tfY7wrZmiTV019qGYSc8Q6b8nKPlqK6sglYPxowJpBrpQ91UM196pzNE45Qx3r";
        let song_title = "Lady Writer";
        let artist_name = "Dire Straits";

        let result = Lyrics::fetch_lyrics(&client, api_key, song_title, artist_name).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
