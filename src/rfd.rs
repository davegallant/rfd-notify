use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Deals {
    topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Topic {
    title: String,
    post_time: String,
    web_path: String,
    topic_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Offer {
    dealer_name: String,
}

#[tokio::main]
pub async fn get_hot_deals() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://forums.redflagdeals.com/api/topics?forum_id=9&per_page=40")
        .await?
        .text()
        .await?;
    Ok(resp)
}

pub fn parse_hot_deals(response: String) -> Deals {
    return serde_json::from_str(&response).unwrap();
}
