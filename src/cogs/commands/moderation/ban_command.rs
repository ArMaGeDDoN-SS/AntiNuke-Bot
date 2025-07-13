use serenity::builder::*;
//use serenity::prelude::*;
//use serenity::model::prelude::*;
use serenity::model::permissions::Permissions;
use serenity::model::application::CommandOptionType;
use serenity::model::application::{ResolvedOption, ResolvedValue};

pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first()
    {
        return format!("{}'s id is {}", user.tag(), user.id);
    } else {
        return "Please provide a valid user".to_string();
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ban").description("Ban the user").description_localized("ru", "Забанить пользователя")
        .dm_permission(false)
        .default_member_permissions(Permissions::BAN_MEMBERS)
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "User who should be banned")
                .required(true)
                .name_localized("ru", "пользователь")
                .description_localized("ru", "Пользователь, которого нужно забанить")
        )
        /*.add_option(
            CreateCommandOption::new(CommandOptionType::String, "reason", "Reason ban")
                .required(false)
                .name_localized("ru", "причина")
                .description_localized("ru", "Причина бана")
        )*/
}
