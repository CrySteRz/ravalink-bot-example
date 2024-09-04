use std::error::Error;
use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateEmbed, CreateInteractionResponseMessage, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};
use serenity::builder::CreateInteractionResponse;
use crate::actions::pause::pause_button;
use crate::actions::leave::leave_button;
use crate::actions::resume::resume_button;
// use crate::actions::back::back_button;
// use crate::actions::skip::skip_button;
// use crate::actions::up::up_button;
// use crate::actions::down::down_button;


pub async fn handle_button_interaction(ctx: &Context, interaction: &ComponentInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    match interaction.data.custom_id.as_str() {
        "pause_button" => {

            pause_button(ctx, interaction).await?;

            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Music Paused!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to increase the volume.")
                )).await?;
            }
        },
        "loop_song_button" => {
            let user = interaction.user.clone(); 
            let embed = CreateEmbed::default()
                .description("Loop Button Pressed!") 
                .author(user.into());

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to loop the song.")
                )).await?;
            }
        },
        "playlist_button" => {
            let user = interaction.user.clone(); 

            let embed = CreateEmbed::default()
                .title("Playlist Panel")
                //.description("📜 | Playlist Panel")
                .color(serenity::model::Colour(119))  // Set a blue color
                .image("https://cdn.discordapp.com/attachments/925414156029538375/1107916980314447982/Banner_2_5.png?ex=66d94f18&is=66d7fd98&hm=8f26073df1af6f222eaab31fd5a58f8564aa592e06f853b9c0bed321739ec18a&")  // URL of your larger image
                .author(user.into());

            let options = vec![
                CreateSelectMenuOption::new("▶️Playlist Play", "playlist_play"),
                CreateSelectMenuOption::new("⭐Favorite Play", "playlist_favorite"),
                CreateSelectMenuOption::new("♻️Playlist Delete", "playlist_delete"),
                CreateSelectMenuOption::new("🔀Playlist Play Shuffle", "playlist_shuffle"),
                CreateSelectMenuOption::new("💾Playlist SaveCurrent", "playlist_save"),
            ];

            let kind = CreateSelectMenuKind::String{options};

            let playlist_select_menu = CreateSelectMenu::new("playlist_select_menu", kind)
                .placeholder("📜 | Playlist Panel")  
                .min_values(1)  // Allow selecting at least 1 option
                .max_values(1);  // Allow selecting only 1 option

            let action_row = CreateActionRow::SelectMenu(playlist_select_menu);

            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed).ephemeral(true)
                    .components(vec![action_row])  // Add the action row containing the select menu
            );

            if let Err(_e) = interaction.create_response(&ctx.http, response).await {
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Failed to show the playlist.")
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
                CreateInteractionResponseMessage::new().embed(embed).ephemeral(true)
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
