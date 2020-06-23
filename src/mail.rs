use crate::config::Config;
use crate::rfd::Posts;
use crate::rfd::Topic;
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

const RFD_FORUMS_BASE_URL: &str = "https://forums.redflagdeals.com";

pub fn send(topic: &Topic, posts: &Posts, expression: &str, config: &Config) {
    let api_key = &config.sendgrid.api_key;
    let sg = SGClient::new(api_key.to_string());

    let html_message = format!(
        "\
    <b>Date:</b> {}
    <br>
    <br>
    <b>Dealer:</b> {}
    <br>
    <br>
    <b>Deal:</b> {}
    <br>
    <br>
    <b>Post:</b> {}\
    <br>
    <br>
    <b>Body:</b> {}
    <br>
    <br>
    <b>Matched by expression:</b> {}
    ",
        topic.post_time,
        topic.offer.dealer_name.as_ref().unwrap_or(&"".to_string()),
        topic.offer.url.as_ref().unwrap_or(&"".to_string()),
        format!("{}/{}", RFD_FORUMS_BASE_URL, topic.web_path),
        posts.posts[0].body,
        expression,
    );

    debug!("{}", html_message);

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
