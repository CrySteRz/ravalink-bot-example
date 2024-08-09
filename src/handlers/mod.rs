pub mod default;
pub mod interaction;
pub mod guild;


use serenity::prelude::*;
use crate::caches::guild::GuildCache;

pub struct Handler {
    pub guild_cache: GuildCache,
}

impl Handler {
    pub fn new(guild_cache: GuildCache) -> Self {
        Handler { guild_cache }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        println!("{} connected!", ready.user.name);
        default::register_global_commands(&ctx).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: serenity::model::application::Interaction) {
        interaction::handle_interaction(ctx, interaction).await;
    }

    async fn guild_create(&self, ctx: Context, guild: serenity::model::guild::Guild, is_new: Option<bool>) {
        guild::handle_guild_create(ctx, guild, is_new, &self.guild_cache).await;
    }

    async fn guild_delete(&self, ctx: Context, incomplete: serenity::model::guild::UnavailableGuild, full: Option<serenity::model::guild::Guild>) {
        guild::handle_guild_delete(ctx, incomplete, full, &self.guild_cache).await;
    }
}
