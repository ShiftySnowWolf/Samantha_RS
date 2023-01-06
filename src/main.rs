mod commands;

extern crate serde;
extern crate rmp_serde;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Member;
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::*;
// use crate::commands::refer::load_referrals;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // Ready event
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} is connected!", data_about_bot.user.name);

        let guild_id = GuildId(
            dotenv::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer")
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::refer::register(command))
        }).await;

        println!("Loaded these commands:");
        for command in &commands.unwrap() {
                println!("-> {}", &command.name);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(),
                "refer" => commands::refer::run(&command, &ctx).await,
                _ => "not implemented :(".to_string()
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {:?}", why)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = dotenv::var("TOKEN").expect("Expected a token in the environment");

    // What the bot intends to do.
    let _intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler).await.expect("Error creating client");

    // Load stored data.
    // load_referrals();

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

pub fn get_guild(guild_id: u64 ) -> GuildId {
    GuildId(guild_id)
}

pub async fn get_member(ctx: &Context, guild: GuildId, user: UserId ) -> Member {
    guild.member(&ctx.http, user).await.expect("Expected a Member")
}