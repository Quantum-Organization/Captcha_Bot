use dotenv::dotenv;
use serenity::{all::GatewayIntents, Client};
use std::env;
use std::collections::HashMap;
use captcha_bot::handlers::event::Handler;
use captcha_bot::models::client_data::Data;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("❌ - Token not found!");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("[❌] - Error creating client!");

    {
        let mut client_data = client.data.write().await;
        client_data.insert::<Data>(HashMap::new());
    }

    client.start().await.expect("[❌] - Error running client!");
}
