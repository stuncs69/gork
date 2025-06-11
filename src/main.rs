use ollama_rs::Ollama;
use serenity::framework::StandardFramework;
// use serenity::framework::standard::macros::group; // Comment out or remove
use serenity::prelude::*;
use std::env;

mod cogs;
mod handler;

// use cogs::ping::*; // Comment out or remove for now
use handler::Handler;

// Comment out or remove the group definition
// #[group]
// #[commands(ping)]
// struct General;

#[tokio::main]
async fn main() {
    let ollama = Ollama::new("ollama.stun.tokyo".to_string(), 443);
    // Load .env file
    if let Err(e) = dotenvy::dotenv() {
        println!("Failed to load .env file: {:?}. Continuing without it.", e);
        // Depending on your needs, you might want to panic here if .env is crucial
        // panic!("Failed to load .env file: {:?}", e);
    }

    println!("Starting bot...");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // We are removing the command framework for now to handle pings directly
    // let framework = StandardFramework::new()
    //     .configure(|c| c.prefix("!")) // remove prefix
    //     .group(&GENERAL_GROUP); // remove group

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { ollama })
        // .framework(framework) // remove framework
        .await
        .expect("Err creating client");

    println!("Bot client created. Connecting to Discord...");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
