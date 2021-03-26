use std::{collections::HashSet, error::Error as StdError};

use cronbot::commands::general::*;
use cronbot::utils;
use serenity::{async_trait, framework::{StandardFramework, standard::macros::group}, http::Http, model::{event::ResumedEvent, gateway::Ready}, prelude::*};

#[group]
#[commands(tax)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Resumed");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let token = env!("DISCORD_TOKEN");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix("~")
                   .delimiter(", "))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
    });

    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }

    Ok(())
}
