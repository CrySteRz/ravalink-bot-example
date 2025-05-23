use ravalink_lib::managers::track_manager::TrackManager;
use ravalink_lib::get_handler_from_interaction_mutable;
use ravalink_lib::serenity::RavalinkKey;
use ravalink_lib::PlayerObject;
use serenity::all::ComponentInteraction;
use serenity::all::CreateCommand;
use serenity::all::Context;
use serenity::all::CommandInteraction;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage; 
use std::error::Error;

pub async fn pause_command(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {


    let mut _handler: Option<PlayerObject> = None;
    get_handler_from_interaction_mutable!(ctx, interaction, _handler);

    match _handler {
        Some(handler) => {
            handler.pause().await.unwrap();
        }
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get manager"))).await?;
            return Ok(());
        }
    }

    Ok(())
}

pub async fn pause_button(ctx: &Context, interaction: &ComponentInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {

    println!("Pause button pressed");
    let mut _handler: Option<PlayerObject> = None;
    get_handler_from_interaction_mutable!(ctx, interaction, _handler);

    match _handler {
        Some(handler) => {
            handler.pause().await.unwrap();
        }
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get manager"))).await?;
            return Ok(());
        }
    }

    Ok(())
    
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pause")
        .description("Pause current track")
}