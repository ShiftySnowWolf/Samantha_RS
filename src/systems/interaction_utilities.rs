use std::borrow::Borrow;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::id::{GuildId, UserId};
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{ User, Member };
//use crate::systems::standard_utilities::report_error;

///
pub fn user_from_option(interaction: &ApplicationCommandInteraction, index: usize) -> Option<&User> {
	let user_option = interaction.data.options
		.get(index).expect("Expected a user object.")
		.resolved.as_ref().expect("Expected a user value.");
	if let CommandDataOptionValue::User(user, _member) = user_option { Some(user.borrow()) } else {
		//report_error("systems::user_from_option");
		return None
	}

}

pub async fn get_member(ctx: &Context, guild: GuildId, user: UserId) -> Member {
	guild.member(&ctx.http, user).await.unwrap()
}

pub async fn ephemeral_message_response(ctx: &Context, interaction: &ApplicationCommandInteraction, message_content: String) {
	if let Err(why) = interaction
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| {
					message.content(message_content).ephemeral(true)
				})
		}).await
	{
		println!("Cannot respond to slash command {:?}", why);
	}
}