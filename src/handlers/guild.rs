use serenity::model::guild::{Guild, UnavailableGuild};
use serenity::prelude::*;
use crate::caches::guild::GuildCache;

pub async fn handle_guild_create(_ctx: Context, guild: Guild, _is_new: Option<bool>, guild_cache: &GuildCache) {
    let mut cache = guild_cache.write().await;
    cache.insert(guild.id, guild.clone());
    println!("Joined guild: {} (ID: {})", guild.name, guild.id);
}

//TODO Test delete guild and leave guild
pub async fn handle_guild_delete(_ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>, guild_cache: &GuildCache) {
    let mut cache = guild_cache.write().await;
    if let Some(full_guild) = full {
        cache.remove(&full_guild.id);
        
        println!("Left guild: {} (ID: {})", full_guild.name, full_guild.id);
    } else {
        cache.remove(&incomplete.id);
        println!("Left guild with ID: {}", incomplete.id);
    }
}
