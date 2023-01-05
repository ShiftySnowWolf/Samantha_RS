use std::collections::HashMap;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::Context;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use serenity::model::id::UserId;
use crate::get_member;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct ReferralData {
    referrals: u8,
    referrers: Vec<UserId>
}

struct Referral {
    referred: HashMap<UserId, ReferralData>
}

pub async fn run(interaction: &ApplicationCommandInteraction, ctx: &Context) -> String {
    let executor = interaction.member.unwrap();

    // The User field in command
    let user_option = interaction.data.options
        .get(0)
        .expect("Expected a user object.")
        .resolved
        .as_ref()
        .expect("Expected a user value.");


    // Get the member who was referred
    let user;
    if let CommandDataOptionValue::User(_user, _) = user_option { user = _user } else { return "Failed command.".to_string(); }
    let member = get_member(&ctx, interaction.guild_id.unwrap(), user.id).await;

    // --- This is a test to see if User can be gotten without using the above code. ---
    let test = interaction.data.options.get(0).unwrap().resolved.unwrap();
    if test != CommandDataOptionValue::User(u, m) { return "User option is not a User".to_string() }
    format!("You have referred {}", member.nick.unwrap())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("refer")
        .description("Refer a user for an org membership.")
        .create_option(|option| {
            option
                .name("user")
                .description("The user that you want to refer.")
                .kind(CommandOptionType::User)
                .required(true)
        })
}