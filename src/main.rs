extern crate serenity;

use serenity::prelude::*;
use serenity::model::*;
use serenity::utils::MessageBuilder;
use std::env;

mod dictionary;

fn main() {
    let res = dictionary::fetch(mw_api_key, String::from("aggressive"));
    println!("Dict def: {}", res);
    let mut client = Client::new(&token, Handler);

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

struct Handler;

impl EventHandler for Handler {
    fn on_message(&self, _: Context, msg: Message) {
        if msg.content == "!ping" {
            let channel = match msg.channel_id.get() {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                },
            };

            // The message builder allows for creating a message by
            // mentioning users dynamically, pushing "safe" versions of
            // content (such as bolding normalized content), displaying
            // emojis, and more.
            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
