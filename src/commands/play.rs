type Error = Box<dyn std::error::Error + Send + Sync>;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed};
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandOptionType;
use serenity::client::Context;
use crate::utils::*;

use ravalink_lib::{ get_handler_from_interaction_mutable, PlayerObject };
use ravalink_lib::serenity::RavalinkKey;
use serenity::model::application::CommandInteraction;
use serenity::model::application::CommandDataOptionValue;
use ravalink_lib::managers::player_manager::Player;
use serenity::all::UserId;
use serenity::model::guild::Guild;
use serenity::all::ChannelId;
use crate::commands::{create_control_buttons, send_interaction_response};
use crate::caches::guild::GuildCacheKey;
use crate::try_or_respond;
use ravalink_lib::managers::channel_manager::ChannelManager;

pub fn get_voice_channel_for_user(guild: &Guild, user_id: &UserId) -> Option<ChannelId> {
    guild.voice_states.get(user_id).and_then(|vs| vs.channel_id)
}


fn create_button(id: &str, label: &str) -> CreateButton {
    CreateButton::new(id).label(label).style(ButtonStyle::Secondary)
}


pub async fn join(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let guild_id = interaction.guild_id.ok_or_else(|| {
        let _ = interaction.create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Failed to get guild"),
            ),
        );
        "Guild not found"
    })?;

    let data = ctx.data.read().await;
    let guild_cache = data.get::<GuildCacheKey>().expect("Expected GuildCache in TypeMap");
    let cache = guild_cache.read().await;
    let guild = try_or_respond!(cache.get(&guild_id).cloned(), ctx, interaction, "Guild not found in cache");
    let member_id = try_or_respond!(interaction.member.as_ref().map(|m| m.user.id), ctx, interaction, "Failed to get member");
    let connect_to = try_or_respond!(get_voice_channel_for_user(&guild, &member_id), ctx, interaction, "You are not in a voice channel");

    let manager = data.get::<RavalinkKey>().expect("Expected RavalinkKey in TypeMap");
    let mx = manager.lock().await;

    match PlayerObject::new(guild_id.into(), mx.tx.clone()).await {
        Ok(mut handler) => {
            handler.connect(connect_to.into()).await?;
            mx.players.write().await.insert(guild_id.to_string(), handler);
        },
        Err(e) => {
            send_interaction_response(ctx, interaction, format!("Failed to create player: {:?}", e)).await?;
        }
    }
    Ok(())
}


pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), Error> {
    let url = interaction.data.options.iter()
        .find(|opt| opt.name == "url")
        .and_then(|opt| match &opt.value {
            CommandDataOptionValue::String(val) => Some(val),
            _ => None,
        }).ok_or("URL option not found")?;
    
    if !validate_url(url) {
        interaction.create_response(
            &ctx.http, 
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Invalid URL")
            )
        ).await?;
        return Ok(());
    }

    let mut handler: Option<PlayerObject> = None;
    get_handler_from_interaction_mutable!(ctx, interaction, handler);
    if handler.is_none() {
        join(ctx, interaction).await?;
        get_handler_from_interaction_mutable!(ctx, interaction, handler);
    }

    if let Some(mut handler) = handler {
        handler.play(url.to_string()).await?;
        let embed = CreateEmbed::default()
            .description(format!("Playing: ({})", url))
            .author(interaction.user.clone().into());
        
            let action_rows = create_control_buttons();
            interaction.create_response(
                &ctx.http, 
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .components(action_rows)
                )
            ).await?;
    } else {
        interaction.create_response(
            &ctx.http, 
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Failed to get manager")
            )
        ).await?;
    }
    
    Ok(())
}

/// Registers the "play" command.
pub fn register() -> CreateCommand {
    CreateCommand::new("play")
        .description("A play command")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "url",
                "URL to play (Youtube | Soundcloud | Spotify)"
            ).required(true)
        )
}
