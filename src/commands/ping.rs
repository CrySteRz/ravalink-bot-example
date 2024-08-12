use std::error::Error;
use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateInteractionResponseMessage};
use serenity::builder::{CreateCommand, CreateInteractionResponse};
use crate::communication::default::RustyMessage;
use crate::communication::default::RustyMessageError;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = interaction.user.clone(); 
    let embed = CreateEmbed::default()
        .description(RustyMessage::Ping.to_string()) 
        .author(user.into()); 

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    );

    if let Err(_e) = interaction.create_response(&ctx.http, response).await {
        interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(RustyMessageError::Ping)
        )).await?;
    }
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
