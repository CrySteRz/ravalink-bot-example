use serenity::all::UserId;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;
use serenity::builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::guild::Guild;
use serenity::all::ChannelId;
use std::error::Error;
use ravalink_lib::PlayerObject;
use ravalink_lib::serenity::RavalinkKey;
use ravalink_lib::managers::channel_manager::ChannelManager;
use crate::caches::guild::GuildCacheKey;
use crate::commands::send_interaction_response;



pub fn get_voice_channel_for_user(guild: &Guild, user_id: &UserId) -> Option<ChannelId> {
    guild.voice_states.get(user_id).and_then(|voice_state| voice_state.channel_id)
}


pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Box<dyn Error + Send + Sync>> {
    let guild_id = match interaction.guild_id {
        Some(guild) => guild,
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get guild"))).await?;
            return Ok(());
        }
    };

    let data = ctx.data.read().await;
    let guild_cache = data.get::<GuildCacheKey>().expect("Expected GuildCache in TypeMap");
    let cache = guild_cache.read().await;

    let guild = match cache.get(&guild_id) {
        Some(guild) => guild.clone(),
        None => {
            send_interaction_response(ctx, interaction, "Guild not found in cache").await?;
            return Ok(());
        }
    };

    let member_id = match &interaction.member {
        Some(member) => member.user.id,
        None => {
            send_interaction_response(ctx, interaction, "Failed to get member").await?;
            return Ok(());
        }
    };

    let channel_id = get_voice_channel_for_user(&guild, &member_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            send_interaction_response(ctx, interaction, "You are not in a voice channel").await?;
            return Ok(());
        }
    };

    let manager = data.get::<RavalinkKey>();
    let mx = manager.unwrap().lock().await;

    // Always create a new player, no check for pre-existing player CHECK HERE
    println!("Creating new player");
    let handler = PlayerObject::new(guild_id.into(), mx.tx.clone()).await;
    println!("Created new handler");

    match handler {
        Ok(mut handler) => {
            handler
                .connect(connect_to.into())
                .await
                .unwrap();
            println!("Joined channel");

            mx.players
                .write()
                .await
                .insert(guild_id.to_string(), handler);
            println!("Inserted new player");
        }
        Err(e) => {
            send_interaction_response(ctx, interaction, format!("Failed to create player: {:?}", e)).await?;
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("A join command")
}
