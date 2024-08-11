use std::error::Error;

use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateInteractionResponseMessage};
use serenity::builder::{CreateCommand, CreateInteractionResponse};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = create_response(interaction, ctx, "Babasha").await {
        interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!("Error occurred: {:?}", e)))).await?;
    }
    Ok(())
}

pub async fn create_response(
    interaction: &CommandInteraction,
    ctx: &Context, 
    message: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let embed = CreateEmbed::default()
        .description(format!("{message}"));
    create_embed_response(interaction, ctx, embed).await
}

pub async fn create_embed_response(
    interaction: &CommandInteraction,
    ctx: &Context, 
    embed: CreateEmbed,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = CreateInteractionResponseMessage::new().add_embed(embed);

    match interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!("Failed to create response: {:?}", e)))).await?;
            Ok(())
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
