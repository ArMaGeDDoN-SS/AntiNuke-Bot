use std::thread;
use std::time::Duration;

use crate::cogs;

use crate::models::whitelist::AntiNukePermissions;

use crate::models::anti_nuke::PunishmentType;
use crate::models::anti_nuke::DamagedTextChannel;
use crate::models::anti_nuke::DamagedVoiceChannel;
use crate::models::anti_nuke::DamagedCategoryChannel;
use crate::models::anti_nuke::DamagedChannelDefault;

use crate::models::anti_nuke::DamagedRole;
use crate::models::anti_nuke::DamagedGuild;
use crate::models::anti_nuke::DamagedUserPunishment;
use crate::models::anti_nuke::get_last_audit_log_user;

use crate::models::punishments::PunishmentUser;

use serenity::async_trait;
use serenity::all::RoleAction;
use serenity::all::MemberAction;
use serenity::all::ChannelAction::{Create, Delete};

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};

use serenity::model::{
    user::User,
    gateway::Ready,
    guild::audit_log::Action,
    channel::{ChannelType, GuildChannel, Message},
    guild::{PartialGuild, Guild, Member, Role}
};
use serenity::model::id::{GuildId, RoleId};
use serenity::builder::{EditRole};
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ban" => Some(cogs::commands::moderation::ban_command::run(&command.data.options()).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }


    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected: https://discord.com/api/oauth2/authorize?client_id=1104115325932404827&permissions=8&scope=bot", ready.user.name);
    }


    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let user_id = member.user.id.get();
        
        if !member.user.bot {
            return
        }

        if AntiNukePermissions::black_list_check(&user_id).await {
            if let Err(_) = PunishmentUser::entry(&ctx, member.guild_id, member.user.id, "Anti-Nuke: Black list bots").await {} else { return };
        }

        let bot_role = &mut member.roles[0].to_role_cached(&ctx.cache).unwrap();
        let old_permissions = member.roles[0].to_role_cached(&ctx.cache).unwrap().permissions;

        bot_role.permissions.set(bot_role.permissions, false);
        bot_role.edit(&ctx.http, EditRole::from_role(&bot_role)).await.unwrap();

        thread::sleep(Duration::from_secs(20));

        match bot_role.edit(&ctx.http, EditRole::new().permissions(old_permissions)).await {
            Ok(()) => return,
            Err(_) => return
        };  
    }


    async fn guild_update(&self, ctx: Context, old_data: Option<Guild>, new_data: PartialGuild) {
        let user = get_last_audit_log_user(&ctx, Some(Action::GuildUpdate), new_data.id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, new_data.id).await {
            return;
        }

        if let Err(_) = PunishmentUser::entry(&ctx, new_data.id, user, "Anti-Nuke: Unauthorized server editing").await { 
            return;
        };

        DamagedGuild::guild_recovery(&ctx, old_data, new_data).await;
    }


    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Member(MemberAction::BanAdd)), guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, guild_id).await {
            return;
        }

        if let Err(_) = PunishmentUser::entry(&ctx, guild_id, user,"Anti-Nuke: Unauthorized banning of members").await {
            return;
        };

        DamagedUserPunishment::user_recovery(&ctx, guild_id, banned_user, PunishmentType::Ban).await;    
    }


    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, _: User, _: Option<Member>) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Member(MemberAction::Kick)), guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, guild_id, user, "Anti-Nuke: Unauthorized kick of members").await { 
            return;
        };
    }


    async fn guild_role_update(&self, ctx: Context, old_data: Option<Role>, new_data: Role) {
        if new_data.managed { 
            return; 
        };

        let old_data = old_data.unwrap();
        let user = get_last_audit_log_user(&ctx, Some(Action::Role(RoleAction::Update)), new_data.guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, new_data.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, new_data.guild_id, user, "Anti-Nuke: Unauthorized roles editing").await { 
            return;
        };

        DamagedRole::role_recovery(&ctx, old_data, false).await;
    }


    async fn channel_update(&self, ctx: Context, old_data: Option<GuildChannel>, new_data: GuildChannel) {
        let old_data = old_data.unwrap();
        
        let user = get_last_audit_log_user(&ctx, None, old_data.guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, new_data.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, new_data.guild_id, user, "Anti-Nuke: Unauthorized channels editing").await { 
            return;
        };

        match new_data.kind {
            ChannelType::Text => DamagedTextChannel::channel_recovery(&ctx, old_data, false).await,
            ChannelType::Voice => DamagedVoiceChannel::channel_recovery(&ctx, old_data, false).await,
            ChannelType::Category => DamagedCategoryChannel::channel_recovery(&ctx, old_data, false).await,
            _ => DamagedChannelDefault::channel_recovery(&ctx, old_data, false).await   
        };
        return;
    }


    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Channel(Create)), channel.guild_id).await;
        
        if AntiNukePermissions::has_permissions(&ctx, user, channel.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, channel.guild_id, user, "Anti-Nuke: Unauthorized channels creation").await { 
            return;
        };

        channel.delete(&ctx.http).await.unwrap();
    }


    async fn channel_delete(&self, ctx: Context, channel: GuildChannel, _messages: Option<Vec<Message>>) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Channel(Delete)), channel.guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, channel.guild_id).await {
            return;
        }

        if let Err(_) = PunishmentUser::entry(&ctx, channel.guild_id, user, "Anti-Nuke: Unauthorized channels deleting").await { 
            return;
        };

        match channel.kind {
            ChannelType::Text => DamagedTextChannel::channel_recovery(&ctx, channel, true).await,
            ChannelType::Voice => DamagedVoiceChannel::channel_recovery(&ctx, channel, true).await,
            _ => DamagedChannelDefault::channel_recovery(&ctx, channel, true).await
        };
    }


    async fn category_create(&self, ctx: Context, category: GuildChannel) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Channel(Create)), category.guild_id).await;
        
        if AntiNukePermissions::has_permissions(&ctx, user, category.guild_id).await{
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, category.guild_id, user, "Anti-Nuke: Unauthorized category-channels create").await { 
            return;
        };

        category.delete(&ctx.http).await.unwrap();
    }


    async fn category_delete(&self, ctx: Context, category: GuildChannel) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Channel(Delete)), category.guild_id).await;

        if AntiNukePermissions::has_permissions(&ctx, user, category.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, category.guild_id, user, "Anti-Nuke: Unauthorized category-channels deleting").await { 
            return;
        };

        DamagedCategoryChannel::channel_recovery(&ctx, category, true).await
    }


    async fn guild_role_create(&self, ctx: Context, mut role: Role) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Role(RoleAction::Create)), role.guild_id).await;
        
        if AntiNukePermissions::has_permissions(&ctx, user, role.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, role.guild_id, user, "Anti-Nuke: Unauthorized roles create").await { 
            return;
        };

        role.delete(&ctx.http).await.unwrap();
    }


    async fn guild_role_delete(&self, ctx: Context, guild_id: GuildId, _role_id: RoleId, role_option: Option<Role>) {
        let user = get_last_audit_log_user(&ctx, Some(Action::Role(RoleAction::Delete)), guild_id).await;

        let role = role_option.unwrap();

        if AntiNukePermissions::has_permissions(&ctx, user, role.guild_id).await {
            return;
        };

        if let Err(_) = PunishmentUser::entry(&ctx, guild_id, user, "Anti-Nuke: Unauthorized roles deleting").await { 
            return;
        };

        DamagedRole::role_recovery(&ctx, role, true).await;
    }
}


/*
    // Action::Member(MemberAction::Update)
    async fn guild_member_update(&self, ctx: Context, old_data: Option<Member>, new_data: Option<Member>, event: GuildMemberUpdateEvent) {
        let user = get_last_audit_log_user(&ctx, None, event.guild_id).await;
        let user_id = user.get();

        println!("{:?}", user_id);
        if AntiNukePermissions::has_permissions(&ctx, user, event.guild_id).await {
            println!("+ Участник {:#?} был измененён", event.nick);
            return;
        }

        println!("- Участник {:#?} был измененён", event.nick);

        DamagedUserUpdate::user_recovery(&ctx, old_data, new_data).await;
    }
*/