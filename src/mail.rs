use crate::config::Config;
use crate::rfd::Topic;
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};

pub fn send(topic: &Topic, config: &Config) {
    let api_key = &config.sendgrid.api_key;

    let sg = SGClient::new(api_key.to_string());

    let html_message = format!(
        "\
    <h3>First Posted: {}</h3>
    <br>
    {}\
    ",
        topic.post_time, topic.web_path
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
