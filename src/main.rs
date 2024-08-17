mod authentication_server;
mod commands;
use commands::{auth, pop, Data};

use poise::serenity_prelude::{self as serenity};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Location {
    location: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    data: Location,
}

#[tokio::main]
async fn main() {
    tokio::spawn(authentication_server::listen_for_auth_responses());

    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![pop(), auth()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .unwrap()
        .start()
        .await
        .unwrap();
}
