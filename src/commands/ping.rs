use std::error::Error;

use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateInteractionResponseMessage};
use serenity::builder::{CreateCommand, CreateInteractionResponse};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    "Hey, I'm alive!".to_string();

    Ok(())
}

pub async fn create_response(
    interaction: &mut CommandInteraction,
    ctx: &Context, 
    message: String,
) -> Result<(), Box<dyn Error + Send + Sync>>{
    let mut embed = CreateEmbed::default();
    let mut test = embed;
    test.description(format!("{message}"));
    create_embed_response(interaction, &ctx, test).await
}

pub async fn create_embed_response(
    interaction: &mut CommandInteraction,
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
            // If creating the job failed send an error message
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!("Failed to create player: {:?}", e)))).await?;
            Ok(())
        }
    }
}


pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}