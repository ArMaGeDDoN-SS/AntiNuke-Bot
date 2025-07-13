use serenity::Error;
use serenity::prelude::SerenityError;
use serenity::model::id::{GuildId, UserId};
use serenity::model::error::Error as ModelError;
use serenity::prelude::*;

pub async fn user_kick(ctx: &Context, guild_id: GuildId, user_id: UserId, reason: &str) -> Result<(), Error> {
    if let Err(err) = guild_id.kick_with_reason(&ctx.http, user_id, reason).await {
        match err {
            SerenityError::Model(ModelError::InvalidUser) => return Ok(()),
            _ => return Err(err)
        };
    };

    Ok(())
}