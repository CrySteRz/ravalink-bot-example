use ravalink_lib::managers::track_manager::TrackManager;
use ravalink_lib::get_handler_from_interaction_mutable;
use ravalink_lib::serenity::RavalinkKey;
use ravalink_lib::PlayerObject;
use serenity::all::CreateCommand;
use serenity::all::Context;
use serenity::all::CommandInteraction;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage; 
use std::error::Error;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut _handler: Option<PlayerObject> = None;
    get_handler_from_interaction_mutable!(ctx, interaction, _handler);

    match _handler {
        Some(handler) => {
            handler.pause().await.unwrap();
            interaction.create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Track paused.")
                )
            ).await?;
        }
        None => {
            interaction.create_response(
                &ctx.http, 
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("No active player found. Use /play to start a track.")
                )
            ).await?;
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pause")
        .description("Pause current track")
}