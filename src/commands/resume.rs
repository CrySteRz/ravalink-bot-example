use charcoal_client::actions::track_manager::TrackManager;
use charcoal_client::get_handler_from_interaction_mutable;
use charcoal_client::serenity::CharcoalKey;
use charcoal_client::PlayerObject;
use serenity::all::CreateCommand;
use serenity::all::Context;
use serenity::all::CommandInteraction;
use serenity::all::CreateInteractionResponse;
use serenity::all::CreateInteractionResponseMessage; 
use std::error::Error;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {


    let mut _handler: Option<&mut PlayerObject> = None;
    get_handler_from_interaction_mutable!(ctx, interaction, _handler);

    match _handler {
        Some(handler) => {
            handler.resume_playback().await.unwrap();
        }
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get manager"))).await?;
            return Ok(());
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("resume")
        .description("Resume current track")
}