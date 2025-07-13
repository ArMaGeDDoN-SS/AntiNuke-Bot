use serenity::Error;
use serenity::prelude::SerenityError;
use serenity::model::error::Error as ModelError;

use serenity::model::guild::Role;
use serenity::builder::{EditRole};
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::*;


pub async fn quarantine(ctx: &Context, guild_id: GuildId, user_id: UserId) -> Result<(), Error> {
	let user = match ctx.http.get_member(guild_id, user_id).await {
		Ok(value) => value,
		Err(err) => return Err(err)
	};

    for role_id in &user.roles {
    	let mut role = role_id.to_role_cached(&ctx.cache).unwrap();

    	if role.managed {
    		role.permissions.set(role.permissions, false);
    		role.edit(&ctx.http, EditRole::from_role(&role)).await.unwrap();
    	};

    	if check_role_permissions(role).await {
    		if let Err(err) = user.remove_role(&ctx.http, role_id).await {
    			match err {
            		SerenityError::Model(ModelError::InvalidUser) => return Ok(()),
            		_ => return Err(err)
    			};
    		};
    	};
    };

    Ok(())
}

/*
	Другие важные функции
*/

async fn check_role_permissions(role: Role) -> bool {
	let pr = role.permissions;

	if pr.administrator() || pr.ban_members() || pr.deafen_members() || pr.kick_members()
	|| pr.manage_channels() || pr.manage_guild() || pr.manage_roles() || pr.manage_webhooks()
	|| pr.mention_everyone() || pr.moderate_members() {
		return true;
	};

	false
}