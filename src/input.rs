use dialoguer::{Input, theme::ColorfulTheme};


pub fn get_str( user_prompt: Option<&str> ) -> String {
    return Input::new()
	.with_prompt(user_prompt.unwrap_or(">> "))
	.interact_text()
	.unwrap();
}

// Idiomatic implementation for now
// Pass none if not any
pub fn get_int( number_prompt: Option<&str> ) -> u64 {
    return Input::with_theme(&ColorfulTheme::default())
	.with_prompt(number_prompt.unwrap_or("Enter a number: "))
	.interact_text()
	.unwrap();
}

