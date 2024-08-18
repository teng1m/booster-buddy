mod authentication_server;
mod commands;
mod sql_manager;

use commands::{auth, pop, Data};

use poise::serenity_prelude::{self as serenity};

#[tokio::main]
async fn main() {
    // TODO you were here
    let id_associations = sql_manager::load_from_db().unwrap_or(vec![]);
    authentication_server::start_response_server();

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
