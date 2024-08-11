
use serenity::all::*;
use std::error::Error;
use crate::communication::default::{RustyMessage, RustyMessageError};

pub async fn create_response(
    interaction: &CommandInteraction,
    ctx: &Context, 
    response: RustyMessage,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let embed = CreateEmbed::default()
        .description(format!("{response}"));
    create_embed_response(interaction, ctx, embed).await
}

pub async fn create_error_response(
    interaction: &CommandInteraction,
    ctx: &Context, 
    error: RustyMessageError,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let embed = CreateEmbed::default()
        .description(format!("Warning: {error}"));
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
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(RustyMessageError::GenericError { error: (Box::new(e)) }))).await?;
            Ok(())
        }
    }
}

// pub async fn custom_player_embed(
//     interaction: &CommandInteraction,
//     ctx: &Context, 
//     player: PlayerObject,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     let embed = CreateEmbed::default()
        
//     create_embed_response(interaction, ctx, embed).await
// }