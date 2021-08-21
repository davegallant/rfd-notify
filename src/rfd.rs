use crate::config::Config;
use crate::db;
use crate::mail;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deals {
    topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct Topic {
    #[serde(rename = "topic_id")]
    pub id: u32,
    pub title: String,
    pub post_time: String,
    pub web_path: String,
    pub offer: Offer,
}

#[derive(Serialize, Deserialize)]
pub struct Offer {
    pub dealer_name: Option<String>,
    pub url: Option<String>,
}

#[tokio::main]
pub async fn get_hot_deals() -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://forums.redflagdeals.com/api/topics?forum_id=9&per_page=40")
        .await?
        .text()
        .await?;
    Ok(resp)
}

#[tokio::main]
pub async fn get_topic(topic_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&format!(
        "https://forums.redflagdeals.com/api/topics/{}/posts?per_page=1&page=1",
        topic_id
    ))
    .await?
    .text()
    .await?;
    Ok(resp)
}

pub fn parse_hot_deals(response: &str) -> Deals {
    serde_json::from_str(response).unwrap()
}

pub fn parse_posts(response: String) -> Posts {
    serde_json::from_str(&response).unwrap()
}

fn hash_deal(topic: &Topic) -> String {
    let digest = format!("{}{}{}", topic.web_path, topic.title, topic.post_time);
    let mut hasher = Sha256::new();
    hasher.input_str(&digest);
    hasher.result_str()
}

pub fn match_deals(deals: Deals, config: Config, dbpath: &str) {
    for topic in deals.topics.iter() {
        for expression in config.expressions.iter() {
            let mut found_match = false;
            let re = RegexBuilder::new(expression)
                .case_insensitive(true)
                .build()
                .unwrap_or_else(|e| panic!("Invalid regex: {}. {}", expression, e));
            if re.is_match(&topic.title) {
                found_match = true;
                info!(
                    "Expression '{}' matched title: {}",
                    expression, &topic.title
                )
            } else if topic.offer.dealer_name.is_some() {
                let dealer_name = topic.offer.dealer_name.as_ref().unwrap();
                if re.is_match(dealer_name) {
                    found_match = true;
                    info!(
                        "Expression '{}' matched dealer: {}",
                        expression, &topic.title
                    )
                }
            }
            if !found_match {
                continue;
            }
            let deal_hash = hash_deal(topic);
            if db::hash_exists(&deal_hash, db::get_config(dbpath)) {
                info!("deal hash '{}' already exists", &deal_hash);
            } else {
                let posts = parse_posts(
                    get_topic(topic.id)
                        .map_err(|err| error!("{:?}", err))
                        .ok()
                        .unwrap(),
                );
                db::insert(&deal_hash, db::get_config(dbpath));
                mail::send(topic, &posts, expression, &config);
            }
        }
    }
}
