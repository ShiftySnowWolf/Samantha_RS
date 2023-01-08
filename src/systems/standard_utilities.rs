use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn _report_error(_origin: &str, _error: Option<&str>, application_interaction: Option<&ApplicationCommandInteraction>) {
	// Is there an interaction? If so, reply that an error occurred
	match application_interaction {
		None => {}
		Some(_interaction) => {

		}
	}
}