mod commands;
mod systems;

extern crate serde;
extern crate rmp_serde;

use std::borrow::Borrow;
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId};
use serenity::prelude::*;
// use crate::commands::refer::load_referrals;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

	// Ready event -> Runs when the client is ready
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
				.create_application_command(|command| commands::referral::register(command))
		}).await;

		println!("Loaded these commands:");
		for command in &commands.unwrap() {
			println!("-> {}", &command.name);
		}
	}

	// InteractionCreate event -> Runs when the client receives an interaction
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

		// Runs when someone runs a slash command.
		if let Interaction::ApplicationCommand(command) = interaction.borrow() {

			// Run the matching command.
			match command.data.name.as_str() {
				"ping" => commands::ping::run(&ctx, &command).await,
				"refer" => commands::refer::run(&ctx, &command).await,
				"referral" => commands::referral::run(&ctx, &command).await,
				_ => {
					if let Err(why) = command
						.create_interaction_response(&ctx.http, |response| {
							response
								.kind(InteractionResponseType::ChannelMessageWithSource)
								.interaction_response_data(|message| {
									message.content(format!("{} -> No registered executor.", command.data.name)).ephemeral(true)
								})
						}).await
					{
						println!("Cannot respond to slash command: {:?}", why);
					}
				}
			}
		}

		// Runs when someone submits a modal.
		if let Interaction::ModalSubmit(_modal) = interaction.borrow() {};
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