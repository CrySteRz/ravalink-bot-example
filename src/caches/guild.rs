use serenity::model::guild::Guild;
use serenity::model::id::GuildId;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub type GuildCache = Arc<RwLock<HashMap<GuildId, Guild>>>;

pub struct GuildCacheKey;

impl serenity::prelude::TypeMapKey for GuildCacheKey {
    type Value = GuildCache;
}