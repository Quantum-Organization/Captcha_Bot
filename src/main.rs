use dotenv::dotenv;
use serenity::all::{CreateAttachment, EditProfile};
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

    let avatar = CreateAttachment::url(&client.http, "https://cdn.dribbble.com/users/6985884/screenshots/15849023/media/6dfb9f3caf75d8b6acc1f9bde6b885fa.gif").await.unwrap();

    let edit = EditProfile::default()
        .avatar(&avatar);

    client.http.edit_profile(&edit).await.unwrap();
    client.start().await.expect("[❌] - Error running client!");
}
