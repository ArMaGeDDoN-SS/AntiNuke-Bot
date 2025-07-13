use serenity::model::user::User;
use serenity::model::guild::{
    audit_log::Action,
    PartialGuild, Guild, Role,

};
use serenity::model::id::{UserId, GuildId};
use serenity::model::channel::{GuildChannel};
use serenity::builder::{CreateChannel, EditChannel, EditRole, EditGuild};
use serenity::prelude::*;


pub async fn get_last_audit_log_user(ctx: &Context, action: Option<Action>, guild_id: GuildId) -> UserId {
    for user_id in ctx.http.get_audit_logs(guild_id, action, None, None, Some(1)).await.unwrap().users.keys() {
        if user_id != &ctx.http.get_guild(guild_id).await.unwrap().owner_id {
            return *user_id;
        }
    }
    return ctx.http.get_guild(guild_id).await.unwrap().owner_id;
}


pub fn default_channel_recovery(channel: GuildChannel) -> CreateChannel<'static> {
    if channel.parent_id.is_none() {
        return CreateChannel::new(channel.name)
            .kind(channel.kind)
            .nsfw(channel.nsfw)
            .position(channel.position)
            .permissions(channel.permission_overwrites);
    };
    return CreateChannel::new(channel.name)
        .kind(channel.kind)
        .nsfw(channel.nsfw)
        .position(channel.position)
        .category(channel.parent_id.unwrap())
        .permissions(channel.permission_overwrites);

}


pub enum PunishmentType {
    Ban,
    Kick,
    Timeout
}


pub struct DamagedChannelDefault;

impl DamagedChannelDefault {
    pub async fn channel_recovery(ctx: &Context, channel: GuildChannel, is_deleted: bool) {
        let guild = channel.guild_id.to_partial_guild(&ctx.http).await.unwrap();

        if is_deleted {
            guild.create_channel(
                &ctx.http,
                default_channel_recovery(channel)
                    .audit_log_reason("Anti-Nuke: Recovering channels")
            ).await.unwrap();
        } else {
            channel.clone().edit(
                &ctx.http,
                EditChannel::new()
                    .name(&channel.name.to_string())
                    .kind(channel.kind)
                    .nsfw(channel.nsfw)
                    .position(channel.position)
                    .permissions(channel.permission_overwrites)
                    .audit_log_reason("Anti-Nuke: Recovering text-channels")
            ).await.unwrap();
        }
    }
}

pub struct DamagedTextChannel;

impl DamagedTextChannel {
    pub async fn channel_recovery(ctx: &Context, channel: GuildChannel, is_deleted: bool) {
        let guild = channel.guild_id.to_partial_guild(&ctx.http).await.unwrap();

        let topic: String = match &channel.topic {
            Some(channel_topic) => channel_topic.to_string(),
            None => "".to_string()
        } ;

        let slow_mode: u16 = match &channel.rate_limit_per_user {
            Some(slow_mode) => *slow_mode,
            None => 0
        } ;

        if is_deleted {
            guild.create_channel(
                &ctx.http,
                default_channel_recovery(channel)
                    .topic(topic)
                    .rate_limit_per_user(slow_mode)
                    .audit_log_reason("Anti-Nuke: Recovering text-channels")
            ).await.unwrap();
        } else {
            channel.clone().edit(
                &ctx.http,
                EditChannel::new()
                    .name(&channel.name.to_string())
                    .kind(channel.kind)
                    .nsfw(channel.nsfw)
                    .position(channel.position)
                    .permissions(channel.permission_overwrites)
                    .topic(topic)
                    .rate_limit_per_user(slow_mode)
                    .audit_log_reason("Anti-Nuke: Recovering text-channels")
            ).await.unwrap();
        }
    }
}

pub struct DamagedVoiceChannel;

impl DamagedVoiceChannel {
    pub async fn channel_recovery(ctx: &Context, channel: GuildChannel, is_deleted: bool) {
        let guild = channel.guild_id.to_partial_guild(&ctx.http).await.unwrap();

        let bitrate: u32 = match channel.bitrate {
            Some(channel_topic) => channel_topic,
            None => 0
        } ;

        let user_limit: u32 = match channel.user_limit {
            Some(limit) => limit,
            None => 0
        } ;

        if is_deleted {
            guild.create_channel(
                &ctx.http,
                default_channel_recovery(channel)
                    .bitrate(bitrate)
                    .user_limit(user_limit)
                    .audit_log_reason("Anti-Nuke: Recovering voice-channels")
            ).await.unwrap();
        } else {
            channel.clone().edit(
                &ctx.http,
                EditChannel::new()
                    .name(&channel.name.to_string())
                    .kind(channel.kind)
                    .nsfw(channel.nsfw)
                    .position(channel.position)
                    .permissions(channel.permission_overwrites)
                    .bitrate(bitrate)
                    .user_limit(user_limit)
                    .audit_log_reason("Anti-Nuke: Recovering voice-channels")
            ).await.unwrap();
        }
    } 
}

pub struct DamagedCategoryChannel;

impl DamagedCategoryChannel {
    pub async fn channel_recovery(ctx: &Context, channel: GuildChannel, is_deleted: bool) {
        let guild = channel.guild_id.to_partial_guild(&ctx.http).await.unwrap();

        if is_deleted {
            guild.create_channel(
                &ctx.http,
                CreateChannel::new(channel.name)
                    .kind(channel.kind)
                    .position(channel.position)
                    .permissions(channel.permission_overwrites)
                    .audit_log_reason("Anti-Nuke: Recovering category-channels")
            ).await.unwrap();
        } else {
            channel.clone().edit(
                &ctx.http,
                EditChannel::new()
                    .name(&channel.name.to_string())
                    .kind(channel.kind)
                    .position(channel.position)
                    .permissions(channel.permission_overwrites)
                    .audit_log_reason("Anti-Nuke: Recovering category-channels")
            ).await.unwrap();
        }
    } 
}

pub struct DamagedRole;

impl DamagedRole {
    pub async fn role_recovery(ctx: &Context, role: Role, is_deleted: bool) {
        let guild = role.guild_id.to_partial_guild(&ctx.http).await.unwrap();

        if is_deleted {
            guild.create_role(
                &ctx.http,
                EditRole::from_role(&role)
                    .audit_log_reason("Anti-Nuke: Recovering roles")
            ).await.unwrap();
        } else {
            guild.edit_role(
                &ctx.http,
                role.id,
                EditRole::from_role(&role)
                    .audit_log_reason("Anti-Nuke: Recovering roles")   
            ).await.unwrap();
        }
    }
}

pub struct DamagedGuild;

impl DamagedGuild {
    pub async fn guild_recovery(ctx: &Context, old_data: Option<Guild>, mut new_data: PartialGuild) {
        let old_data = old_data.unwrap();

        new_data.edit(
            &ctx.http,
            EditGuild::new()
                .name(old_data.name)
        ).await.unwrap();
    }
}

pub struct DamagedUserPunishment;

impl DamagedUserPunishment {
    pub async fn user_recovery(ctx: &Context, guild_id: GuildId, user: User, punishment_type: PunishmentType) {
        let guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

        match punishment_type {
            PunishmentType::Ban => {
                guild.unban(&ctx.http, user.id).await.unwrap();
            },
            PunishmentType::Kick => {
                return;
            },
            PunishmentType::Timeout => {
                return;
            }
        };
    }
}

/*pub struct DamagedUserUpdate;

impl DamagedUserUpdate {
    pub async fn user_recovery(ctx: &Context, old_data: Option<Member>, new_data: Option<Member>) {
        let old_data = old_data.unwrap();
        let mut new_data = new_data.unwrap();

        new_data.edit(
            &ctx.http,
            EditMember::new()
                .nickname(old_data.nick.unwrap())
                .roles(old_data.roles)
        ).await.unwrap();
    }
}*/