use poise::serenity_prelude::{self as serenity, CreateMessage};
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
use serde::Deserialize;

/// pop a booster
#[poise::command(slash_command)]
async fn pop(
    ctx: Context<'_>,
    #[description = "Reserve Type"]
    #[choices("credit", "free xp", "crew xp", "tank xp")]
    reserve: &'static str,
    #[description = "Reserve Tier"]
    #[choices(10,9,8,7,6,5,4,3,2,1)]
    tier: u8
) -> Result<(), Error> {
    // ctx.say(format!("user account created at {}", ctx.author().created_at())).await?;
    ctx.say(format!("popped a tier {} {} reserve 😎", tier, reserve)).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Location {
    location: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    data: Location,
}

/// authenticate your wargaming account
#[poise::command(slash_command)]
async fn auth(
    ctx: Context<'_>,
    #[description = "Region"]
    #[choices("NA", "EU", "ASIA")]
    region: &'static str,
) -> Result<(), Error> {

    let extension = match region {
        "EU" => "eu",
        "ASIA" => "asia",
        _ => "com"
    };

    let url = format!("https://api.worldoftanks.{}/wot/auth/login/?application_id=ef746aff128156bd7446a669f673bef7&display=page&expires_at=1030000&nofollow=1", extension);

    let res = reqwest::get(url).await?;

    if res.status().is_success() {
        // Parse the JSON response
        let api_response = res.json::<AuthResponse>().await?;

        let msg = CreateMessage::new()
            .content(format!("[Click to authenticate!]({})", api_response.data.location));
    
        ctx.author().direct_message(&ctx.serenity_context(), msg).await?;
    
        ctx.say("Check your DMs for an authentication link!").await?;

    } else {
        println!("Request failed with status: {}", res.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework= poise::Framework::builder()
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

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}