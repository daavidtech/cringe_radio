use serde::{Deserialize, Serialize};

pub async fn get_video_watch_url(video_id: &str, api_key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=snippet",
        video_id,
        api_key
    );
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    let items = &resp["items"];
    let item = items.as_array().unwrap().get(0).unwrap();
    let video_id = item["id"].as_str().unwrap().to_string();
    let watch_url = format!("https://www.youtube.com/watch?v={}", video_id);
    Ok(watch_url)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub thumbnail_url: String,
}

async fn search_youtube(query: &str, api_key: &str) -> anyhow::Result<Vec<SearchResult>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&key={}&maxResults=10&type=video",
        query,
        api_key
    );
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    // println!("{:?}", resp);

    let items = &resp["items"];
    let mut results = Vec::new();
    for item in items.as_array().unwrap() {
        let id = item["id"]["videoId"].as_str().unwrap().to_string();
        let title = item["snippet"]["title"].as_str().unwrap().to_string();
        let thumbnail_url = item["snippet"]["thumbnails"]["medium"]["url"].as_str().unwrap().to_string();
        results.push(SearchResult { id, title, thumbnail_url });
    }
    Ok(results)
}

pub struct Youtube {
    api_key: String,
}

impl Youtube {
    pub fn new(api_key: &str) -> Self {
        Self { 
            api_key: api_key.to_string(), 
        }
    }

    pub async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        search_youtube(query, &self.api_key).await
    }

    pub async fn get_video_watch_url(&self, video_id: &str) -> Result<String, reqwest::Error> {
        get_video_watch_url(video_id, &self.api_key).await
    }
}