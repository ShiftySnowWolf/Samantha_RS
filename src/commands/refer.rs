use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::prelude::Context;
use serde::{Deserialize, Serialize};
use rmp_serde::{Serializer};
use serenity::model::id::UserId;
use crate::get_member;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ReferralData {
    referrals: u8,
    referrers: Vec<UserId>
}

// static mut REFERRALS: HashMap<UserId, ReferralData> = HashMap::new();

// pub fn load_referrals() {
// }

fn create_referral(referred: UserId, referrer: UserId) {
    // Create referral data
    let referral = ReferralData {
        referrals: 1,
        referrers: vec![referrer],
    };

    // Create path for new referral
    let user_file = format!("{}.msgpack", referred);
    let user_path = Path::join("./src/data/referrals".as_ref(), user_file);
    fs::File::create(&user_path).expect(&*format!("Failed to create {}'s referral data.", referred));

    // Serialize referral data to be stored in file.
    let mut buf = Vec::new();
    referral.serialize(&mut Serializer::new(&mut buf)).unwrap();

    // Write serialized data to file.
    let mut referral_file = fs::File::create(&user_path).expect("Failed to write serialized referral data to file.");
    referral_file.write(&*buf).unwrap();
}

async fn update_referral(referred: UserId, referrer: UserId) {
}

fn has_referral(user: UserId) -> bool {
    let user_file = format!("{}.msgpack", user);
    let user_path = Path::join("./src/data/referrals".as_ref(), user_file);
    Path::exists(&*user_path)
}

async fn erase_referral(user: UserId) {
    let user_file = format!("{}.msgpack", user);
    let user_path = Path::join("./src/data/referrals".as_ref(), user_file);
    fs::remove_file(user_path).expect(&*format!("Failed to erase {}'s referral data.", user));
}

pub async fn run(interaction: &ApplicationCommandInteraction, ctx: &Context) -> String {
    let executor = interaction.member.as_ref().unwrap();

    // The User field in command
    let user_option = interaction.data.options
        .get(0).expect("Expected a user object.")
        .resolved.as_ref().expect("Expected a user value.");



    // Get the member who was referred
    let user;
    if let CommandDataOptionValue::User(usr, None) = user_option { user = usr } else { return "Failed to get user".to_string() }
    let member = get_member(&ctx, interaction.guild_id.unwrap(), user.id).await;
    println!("{}", &member.nick.as_ref().unwrap());
    if has_referral(user.id) {

    } else {
        create_referral(user.id, executor.user.id);
    };

    "This command is currently WIP".to_string()
    // format!("You have referred {}", &member.nick.as_ref().unwrap())
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