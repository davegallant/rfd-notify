use crate::config::Config;
use crate::db;
use crate::mail;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use regex::RegexBuilder;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Deals {
    topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub title: String,
    pub post_time: String,
    pub web_path: String,
    topic_id: u32,
    offer: Offer,
}

#[derive(Serialize, Deserialize, Debug)]
struct Offer {
    dealer_name: Option<String>,
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

fn hash_deal(topic: &Topic) -> String {
    let digest = format!("{}{}{}", topic.web_path, topic.title, topic.post_time);
    let mut hasher = Sha256::new();
    hasher.input_str(&digest);
    let hash = hasher.result_str();

    return hash;
}

pub fn match_deals(deals: Deals, config: Config) {
    for topic in deals.topics.iter() {
        for expression in config.expressions.iter() {
            let mut found_match = false;
            let re = RegexBuilder::new(expression)
                .case_insensitive(true)
                .build()
                .expect(&format!("Invalid regex {}", expression));
            if re.is_match(&topic.title) {
                found_match = true;
                debug!(
                    "Expression '{}' matched title: {}",
                    expression, &topic.title
                )
            } else {
                if topic.offer.dealer_name.is_some() {
                    let dealer_name = topic.offer.dealer_name.as_ref().unwrap();
                    if re.is_match(&dealer_name) {
                        found_match = true;
                        debug!(
                            "Expression '{}' matched dealer: {}",
                            expression, &topic.title
                        )
                    }
                }
            }
            if !found_match {
                continue;
            }
            let deal_hash = hash_deal(topic);
            if db::hash_exists(&deal_hash) {
                debug!("deal hash '{}' already exists", &deal_hash);
            } else {
                db::insert(&deal_hash);
                mail::send(topic, &config);
            }
        }
    }
}
