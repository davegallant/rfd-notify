use crate::config::Config;
use crate::rfd::Posts;
use crate::rfd::Topic;
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

const RFD_FORUMS_BASE_URL: &'static str = "https://forums.redflagdeals.com";

pub fn send(topic: &Topic, posts: &Posts, config: &Config) {
    let api_key = &config.sendgrid.api_key;
    let sg = SGClient::new(api_key.to_string());

    let html_message = format!(
        "\
    <b>First Posted:</b> {}
    <br>
    <b>DEALER:</b> {:?}
    <br>
    <b>DEAL:</b> {:?}
    <br>
    <b>POST:</b> {}\
    <br>
    <br>
    <b>Body:</b> {}
    ",
        topic.post_time,
        topic.offer.dealer_name,
        topic.offer.url,
        format!("{}/{}", RFD_FORUMS_BASE_URL, topic.web_path),
        posts.posts[0].body,
    );

    let mail_info = Mail::new()
        .add_to(Destination {
            address: &config.sendgrid.mail_to,
            name: "you",
        })
        .add_from(&config.sendgrid.mail_from)
        .add_subject(&topic.title)
        .add_html(&html_message);

    match sg.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {}", body),
    };
}
