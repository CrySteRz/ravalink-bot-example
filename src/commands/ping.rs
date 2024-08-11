use std::error::Error;
use crate::communication::embed::create_response;
use serenity::all::{CommandInteraction, Context, CreateInteractionResponseMessage};
use serenity::builder::{CreateCommand, CreateInteractionResponse};
use crate::communication::default::RustyMessage;
use crate::communication::default::RustyMessageError;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = create_response(interaction, ctx, RustyMessage::Ping).await {
        interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(RustyMessageError::Ping))).await?;
    }
    Ok(())
}


pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
