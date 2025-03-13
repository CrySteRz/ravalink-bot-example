
use ravalink_lib::managers::channel_manager::ChannelManager;
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
    // Get the guild ID from the interaction.
    let guild_id = interaction.guild_id.ok_or("Guild not found")?;
    
    // Access your shared data to get the central player manager.
    let data = ctx.data.read().await;
    let manager = data.get::<RavalinkKey>().expect("Expected RavalinkKey in TypeMap");
    let mut mx = manager.lock().await;
    
    // Remove the player object for this guild.
    let removed_handler = mx.players.write().await.remove(&guild_id.to_string());
    
    if let Some(mut handler) = removed_handler {
        // Optionally disconnect/cleanup the player if such a method exists.
        handler.stop().await?;
        interaction.create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Player stopped and reset.")
            )
        ).await?;
    } else {
        interaction.create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("No active player found.")
            )
        ).await?;
    }
    
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop")
        .description("Stop command")
}