use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::client::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
	if let Err(why) = interaction
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|message| {
					message.content("Hey there, I'm alive!").ephemeral(true)
				})
		}).await
	{
		println!("Cannot respond to slash command: {:?}", why);
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command
		.name("ping")
		.description("Replies to you.")
}