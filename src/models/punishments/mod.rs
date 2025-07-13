mod punish_functions;

use crate::models::punishments::punish_functions::ban::user_ban;
use crate::models::punishments::punish_functions::kick::user_kick;
use crate::models::punishments::punish_functions::quarantine::quarantine;

use serenity::Error;
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::*;


async fn emergency_punishment(ctx: &Context, guild_id: GuildId, user_id: UserId, reason: &str) -> Result<(), Error> {
	if let Ok(()) = user_ban(&ctx, guild_id, user_id, reason).await {
		return Ok(());
	};
	if let Ok(()) = user_kick(&ctx, guild_id, user_id, reason).await {
		return Ok(());
	};
	if let Err(err) = quarantine(&ctx, guild_id, user_id).await {
		return Err(err);
	};

	Ok(())
}

pub struct PunishmentUser;

impl PunishmentUser {
	pub async fn entry(ctx: &Context, guild_id: GuildId, user_id: UserId, _reason: &str) -> Result<(), Error> {
		/*
			Данный кусок кода должен взять значение из базы данных
		*/

		let status = match 3 {
			1 => user_ban(&ctx, guild_id, user_id, _reason).await,
			2 => user_kick(&ctx, guild_id, user_id, _reason).await,
			_ => quarantine(&ctx, guild_id, user_id).await
		};

		if status.is_ok() {
			return Ok(());
		};

		println!("Аварийное наказание");

		if let Err(err) = emergency_punishment(&ctx, guild_id, user_id, _reason).await {
			return Err(err);
		};

		Ok(())
	}
}