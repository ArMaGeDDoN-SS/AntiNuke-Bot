mod models;
use crate::models::anti_nuke::AntiNukeEntry;
use crate::models::whitelist::AntiNukePermissions;
use crate::models::anti_nuke::get_last_audit_log_user;

use std::thread;
use std::time::Duration;

use url::Url;

//use serenity::model::guild::Integration;
use serenity::async_trait;
//use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::all::ChannelAction::{Create, Update, Delete};
use serenity::all::RoleAction;
use serenity::model::{
    //application::{Interaction},
    channel::{GuildChannel, Message},
    guild::audit_log::{Action},
    guild::{Member, Role},
};

// use serenity::client::ClientBuilder;
use serenity::all::EditChannel;
use serenity::builder::{CreateChannel, EditRole};
use serenity::model::id::{GuildId, UserId, RoleId};
use serenity::prelude::*;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected: https://discord.com/api/oauth2/authorize?client_id=1104115325932404827&permissions=8&scope=bot", ready.user.name);
    }


    /*async fn guild_update(ctx: Context, old_data_if_available: Option<Guild>, new_data: PartialGuild) {

    }*/

    async fn guild_member_addition(&self, ctx: Context, member: Member) {

        let user_id = member.user.id.get();
        
        if member.user.bot {
            if AntiNukePermissions::black_list_check(&user_id).await {
                member.ban_with_reason(
                    &ctx.http, 
                    0, 
                    "Anti-Nuke: Black list bots"
                ).await.unwrap();


            } else {
                let bot_role = &mut member.roles[0].to_role_cached(&ctx.cache).unwrap();
                let old_permissions = member.roles[0].to_role_cached(&ctx.cache).unwrap().permissions;

                bot_role.permissions.set(bot_role.permissions, false);
                bot_role.edit(&ctx.http, EditRole::from_role(&bot_role)).await.unwrap();

                thread::sleep(Duration::from_secs(20));

                bot_role.edit(&ctx.http, EditRole::new().permissions(old_permissions)).await.unwrap();  
            }
        }
    }

    /* 
        *События создания/удаления
    */

    async fn channel_create(&self, ctx: Context, channel: GuildChannel) {

        let user = get_last_audit_log_user(&ctx, Action::Channel(Create), channel.guild_id).await;
        let user_id = user.get();
        
        if AntiNukePermissions::has_permissions(&ctx, user, channel.guild_id).await {
            println!("+ Позиция созданного канала {:?} :{:?}", channel.name, channel.position);
            return;

        } else {
            println!("- Позиция созданного канала {:?} :{:?}", channel.name, channel.position);

            match channel.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Creating a spam channels"
            ).await {
                Ok(()) => {
                    channel.delete(&ctx.http).await.unwrap()
                },
                Err(_) => {
                    channel.delete(&ctx.http).await.unwrap()
                }
            };
        };
    }

    async fn channel_delete(&self, ctx: Context, channel: GuildChannel, _messages: Option<Vec<Message>>) {

        let user = get_last_audit_log_user(&ctx, Action::Channel(Delete), channel.guild_id).await;
        let user_id = user.get();

        if AntiNukePermissions::has_permissions(&ctx, user, channel.guild_id).await {
            println!("+ Позиция удалённого канала {:?} :{:?}", channel.name, channel.position);
            return;

        } else {
            println!("- Позиция удалённого канала {:?} :{:?}", channel.name, channel.position);

            match channel.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Deleting channels"
            ).await {
                Ok(()) => {
                    channel.guild_id.create_channel(
                        &ctx.http, 
                        AntiNukeEntry::channel_recovery(channel).await
                    ).await.unwrap();      
                },
                Err(_) => {
                    channel.guild_id.create_channel(
                        &ctx.http, 
                        AntiNukeEntry::channel_recovery(channel).await
                    ).await.unwrap(); 
                }
            };
        };
    }


    async fn category_create(&self, ctx: Context, category: GuildChannel) {

        let user = get_last_audit_log_user(&ctx, Action::Channel(Create), category.guild_id).await;
        let user_id = user.get();
        
        if AntiNukePermissions::has_permissions(&ctx, user, category.guild_id).await{
            println!("+ Позиция созданной категории {:?} :{:?}", category.name, category.position);
            return;

        } else {
            println!("- Позиция созданной категории {:?} :{:?}", category.name, category.position);

            match category.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Creating a spam channels"
            ).await {
                Ok(()) => {
                    category.delete(&ctx.http).await.unwrap();
                },
                Err(_) => {
                    category.delete(&ctx.http).await.unwrap();
                }
            };
        };
    }


    async fn category_delete(&self, ctx: Context, category: GuildChannel) {

        let user = get_last_audit_log_user(&ctx, Action::Channel(Delete), category.guild_id).await;
        let user_id = user.get();

        if AntiNukePermissions::has_permissions(&ctx, user, category.guild_id).await {
            println!("+ Позиция удалённой категории {:?} :{:?}", category.name, category.position);
            return;

        } else {
            println!("- Позиция удалённой категории {:?} :{:?}", category.name, category.position);
            
            match category.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Deleting channels"
            ).await {
                Ok(()) => {
                    category.guild_id.create_channel(&ctx.http, CreateChannel::new(category.name)
                        .kind(category.kind)
                        .position(category.position)
                        .nsfw(category.nsfw)
                        .permissions(category.permission_overwrites)
                    ).await.unwrap();
                },
                Err(_) => {
                    category.guild_id.create_channel(&ctx.http, CreateChannel::new(category.name)
                        .kind(category.kind)
                        .position(category.position)
                        .nsfw(category.nsfw)
                        .permissions(category.permission_overwrites)
                    ).await.unwrap();            
                }
            };
        };
    }


    async fn guild_role_create(&self, ctx: Context, mut role: Role) {
        let user = get_last_audit_log_user(&ctx, Action::Role(RoleAction::Create), role.guild_id).await;
        let user_id = user.get();
        
        if AntiNukePermissions::has_permissions(&ctx, user, role.guild_id).await {
            println!("+ Позиция созданной роли {:?} :{:?}", role.name, role.position);
            return;

        } else {
            println!("- Позиция созданной роли {:?} :{:?}", role.name, role.position);

            match role.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Creating a spam roles"
            ).await {
                Ok(()) => {
                    role.delete(&ctx.http).await.unwrap();
                },
                Err(_) => {
                    role.delete(&ctx.http).await.unwrap();
                }
            };
        };
    }


    async fn guild_role_delete(&self, ctx: Context, guild_id: GuildId, _role_id: RoleId, role_option: Option<Role>) {
        let user = get_last_audit_log_user(&ctx, Action::Role(RoleAction::Delete), guild_id).await;
        let user_id = user.get();

        let role = role_option.unwrap();

        if AntiNukePermissions::has_permissions(&ctx, user, role.guild_id).await {
            println!("+ Позиция удалённой роли {:?} :{:?}", role.name, role.position);
            return;

        } else {
            println!("- Позиция удалённой роли {:?} :{:?}", role.name, role.position);
        
            match role.guild_id.ban_with_reason(
                &ctx.http, 
                UserId::new(user_id), 
                0, 
                "Anti-Nuke: Deleting roles"
            ).await {
                Ok(()) => {
                    role.guild_id.create_role(&ctx.http, EditRole::from_role(&role)).await.unwrap();
                },
                Err(_) => {
                    role.guild_id.create_role(&ctx.http, EditRole::from_role(&role)).await.unwrap();
                }
            }
        }
    }
}


    /*async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
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
    }*/

    /*
    async fn channel_delete(&self, ctx: Context, channel: GuildChannel, _messages: Option<Vec<Message>>) {
        channel.guild_id.create_channel(ctx.http, CreateChannel::new(channel.name)
            .kind(channel.kind)
        ).await;
    }*/