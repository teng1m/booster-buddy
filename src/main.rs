use poise::serenity_prelude as serenity;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework= poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![pop()],
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