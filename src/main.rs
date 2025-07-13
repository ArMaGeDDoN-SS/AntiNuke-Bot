use AntiNuke-Bot::models::bot_account::Handler;

use url::Url;

use serenity::model::gateway::ActivityType;
use serenity::gateway::ActivityData;
use serenity::prelude::*;


#[tokio::main]
async fn main() {
    let token = "MTEwNDExNTMyNTkzMjQwNDgyNw.G64G-w.zuv1bYNIPrJsTjWlys0zNpq8phUZNksteT7ap4";

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MODERATION; 

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .activity(ActivityData {
            name: "Test Name".to_string(),
            kind: ActivityType::Streaming,
            state: None,
            url: Some(Url::parse("https://youtube.com/watch?v=jscmSYUNGfE").unwrap())
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
