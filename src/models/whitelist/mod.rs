extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use serde::{Deserialize};

use serenity::model::id::{UserId, GuildId};
use serenity::prelude::*;


#[derive(Deserialize)]
pub struct AntiNukePermissions {
    white_list_bots: Vec<u64>,
    black_list_bots: Vec<u64>
}


impl AntiNukePermissions {

    pub async fn black_list_check(bot_id: &u64) -> bool {
        let file = File::open(Path::new("./src/config/config.json")).expect("Unable to open file");
        let reader = BufReader::new(file);

        let config: AntiNukePermissions = serde_json::from_reader(reader).expect("Unable to parse JSON");

        if config.black_list_bots.contains(bot_id) {
            return true
        } else {
            return false
        };
    }
}

impl AntiNukePermissions {

    pub async fn has_permissions(ctx: &Context, user_id: UserId, guild_id: GuildId) -> bool {
        let file = File::open(Path::new("src/config/config.json")).expect("Unable to open file");
        let reader = BufReader::new(file);

        let config: AntiNukePermissions = serde_json::from_reader(reader).expect("Unable to parse JSON");

        let guild = guild_id.to_partial_guild(&ctx.http).await.expect("Ошибка");

        if user_id == guild.owner_id || user_id == UserId::new(1104115325932404827) || config.white_list_bots.contains(&user_id.get()) {
            return true
        } else {
            return false
        };
    }
}


    /*pub async fn white_list_check(bot_id: &u64) -> bool {
    
        let file = File::open("src\\config\\config.json").expect("Unable to open file");
        let reader = BufReader::new(file);

        let config: AntiNukePermissions = serde_json::from_reader(reader).expect("Unable to parse JSON");

        if config.white_list_bots.contains(bot_id) {
            true
        } else {
            false
        }   
    }*/

    /*pub async fn has_permission(ctx: &Context, user_id: UserId, guild_id: GuildId) -> bool {

        let guild = guild_id.to_partial_guild(&ctx.http).await.expect("Ошибка");

        if user_id == guild.owner_id || user_id == UserId::new(1104115325932404827) {
            return true
        } else {
            return false
        };
    }*/