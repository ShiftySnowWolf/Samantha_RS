use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction};
use serenity::client::Context;
use crate::systems::interaction_utilities::*;
use crate::systems::referral_management::*;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
	// Get the member who was referred
	let executor = interaction.member.as_ref().unwrap();

	let user = user_from_option(interaction, 0).expect("Failed to get user from options");
	let member = get_member(&ctx, interaction.guild_id.unwrap(), user.id);

	// Handle the ReferralData belonging to the referred User
	if has_referral(user.id) {
		update_referral(user.id, executor.user.id);
	} else {
		create_referral(user.id, executor.user.id);
	}

	// Respond to the command executor
	ephemeral_message_response(&ctx, interaction, format!("{} has been referred for a membership.", member.await.nick.as_ref().unwrap())).await;
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