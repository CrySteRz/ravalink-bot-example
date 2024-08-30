use std::error::Error;
use serenity::all::{ComponentInteraction, Context, CreateEmbed, CreateInteractionResponseMessage};
use serenity::builder::CreateInteractionResponse;
use crate::actions::pause::pause_button;
use crate::actions::leave::leave_button;


pub async fn handle_button_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    match interaction.data.custom_id.as_str() {
        "pause_button" => {

            pause_button(ctx, interaction).await?;

            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Music Paused!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to pause the music.")
                )).await?;
            }
        },
        "ping_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Pong!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to respond with Pong.")
                )).await?;
            }
        },
        "stop_button" => {

            leave_button(ctx, interaction).await?;
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Music Stopped!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to stop the music.")
                )).await?;
            }
        },
        _ => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Unknown button interaction.")
            )).await?;
        }
    }
    Ok(())
}
