use serenity::all::UserId;
use serenity::client::Context;
use serenity::model::application::CommandInteraction;
use serenity::builder::{CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::guild::Guild;
use serenity::all::ChannelId;
use std::error::Error;
use charcoal_client::PlayerObject;
use charcoal_client::serenity::CharcoalKey;
use charcoal_client::actions::channel_manager::ChannelManager;
use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Metadata;
use crate::caches::guild::GuildCacheKey;
use charcoal_client::actions::standard::CharcoalEventHandler;

//Todo move this
struct CustomEventHandler {}

impl CharcoalEventHandler for CustomEventHandler {
    fn handle_error(&self, error_report: ErrorReport) {
        println!("Uh oh got error in event handler: {:?}", error_report);
    }

    fn handle_metadata_response(&self, metadata: Metadata) {
        println!("Got metadata back in event handler: {:?}", metadata);
    }
}

pub fn get_voice_channel_for_user(guild: &Guild, user_id: &UserId) -> Option<ChannelId> {
    guild
        .voice_states
        .get(user_id)
        .and_then(|voice_state| voice_state.channel_id)
}
//Till here


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
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Guild not found in cache"))).await?;
            return Ok(());
        }
    };
    
    let member_id = match &interaction.member {
        Some(member) => member.user.id,
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get member"))).await?;
            return Ok(());
        }
    };

    let channel_id = get_voice_channel_for_user(&guild, &member_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("Failed to get channel"))).await?;
            return Ok(());
        }
    };
    let r = ctx.data.read().await;
    let manager = r.get::<CharcoalKey>();
    let mx = manager.unwrap().lock().await;


    // Check if we have already created the player by checking if the player's GuildID exists in the Players HashMap
    // Stored inside of the Charcoal Instance.
    // If we have already created the player just join the channel
    if mx.players.read().await.contains_key(&guild_id.to_string()) {
        println!("Using pre-existing player");
        // Get a write lock on the players HashMap
        let mut players = mx.players.write().await;
        // Get a mutable reference to said player
        let handler = players.get_mut(&guild_id.to_string()).expect(
            "This should never happen because we checked the key exists in the if check above",
        );
        // Join the channel
        handler
            .join_channel(connect_to.to_string(), false)
            .await
            .unwrap(); // We use false here so Charcoal does not create a pre-existing job
    } else {
        println!("Creating new player");
        // If we have not created the player create it and then join the channel
        let handler = PlayerObject::new(guild_id.to_string(), mx.tx.clone()).await;
        println!("Created new handler");
        // Make sure creating the PlayerObject worked
        match handler {
            Ok(mut handler) => {
                // Register an error callback so errors from the hearth server can be reported back to us
                handler.register_event_handler(CustomEventHandler {}).await;
                // Join the channel
                println!("Registered error callback");
                handler
                    .join_channel(connect_to.to_string(), true)
                    .await
                    .unwrap(); // We use true here to tell Charcoal to create the Job
                println!("Joined channel");
                // Insert the newly created PlayerObject into the HashMap so we can use it later
                mx.players
                    .write()
                    .await
                    .insert(guild_id.to_string(), handler);
                println!("Inserted new player");
            }
            Err(e) => {
                // If creating the job failed send an error message
                interaction.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(format!("Failed to create player: {:?}", e)))).await?;
            }
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join")
        .description("A join command")
}
