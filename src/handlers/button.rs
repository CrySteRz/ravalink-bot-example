use std::error::Error;
use serenity::all::{ComponentInteraction, Context, CreateEmbed, CreateInteractionResponseMessage};
use serenity::builder::CreateInteractionResponse;
use crate::actions::pause::pause_button;
use crate::actions::leave::leave_button;
use crate::actions::resume::resume_button;


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
        "resume_button" => {

            resume_button(ctx, interaction).await?;

            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Music Resumed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to resume the music.")
                )).await?;
            }
        },
        "back_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Back Button Pressed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to go back.")
                )).await?;
            }
        },
        "skip_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Skip Button Pressed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to skip the music.")
                )).await?;
            }
        },
        "down_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Down Button Pressed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to decrease the volume.")
                )).await?;
            }
        },
        "up_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Up Button Pressed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to increase the volume.")
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
