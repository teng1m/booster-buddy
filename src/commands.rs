use poise::serenity_prelude::CreateMessage;

type Error = Box<dyn std::error::Error + Send + Sync>;
pub struct Data;
type Context<'a> = poise::Context<'a, Data, Error>;

/// pop a booster
#[poise::command(slash_command)]
pub async fn pop(
    ctx: Context<'_>,
    #[description = "Reserve Type"]
    #[choices("credit", "free xp", "crew xp", "tank xp")]
    reserve: &'static str,
    #[description = "Reserve Tier"]
    #[choices(10, 9, 8, 7, 6, 5, 4, 3, 2, 1)]
    tier: u8,
) -> Result<(), Error> {
    // ctx.say(format!("user account created at {}", ctx.author().created_at())).await?;
    ctx.say(format!("popped a tier {} {} reserve 😎", tier, reserve))
        .await?;
    Ok(())
}

/// authenticate your wargaming account
#[poise::command(slash_command)]
pub async fn auth(
    ctx: Context<'_>,
    #[description = "Region"]
    #[choices("NA", "EU", "ASIA")]
    region: &str,
) -> Result<(), Error> {
    let extension = match region {
        "EU" => "eu",
        "ASIA" => "asia",
        _ => "com",
    };

    let url = format!("https://api.worldoftanks.{}/wot/auth/login/?application_id=bd09ad6840803e46880ac67012edf241&redirect_uri=localhost:5000", extension);

    ctx.say("Check your DMs for an authentication link!")
        .await?;

    let msg = CreateMessage::new().content(format!("[Click to authenticate!]\n\n{}", url));

    ctx.author()
        .direct_message(&ctx.serenity_context(), msg)
        .await?;

    Ok(())
}
