use core::str;
use std::{io, str::FromStr};

pub fn get_parameter<R: io::BufRead, O: FromStr>(args: &[&str], index: usize, stdin: &mut R) -> O {
	// Check inside the argument list if the plugin version is present
	let plugin_version = get_app_argument::<O>(args, index);
	match plugin_version {
		Some(version) => return version,
		None => println!("Please enter the plugin version:")
	}

	get_user_input(stdin)
}

fn get_app_argument<T: FromStr>(args: &[&str], index: usize) -> Option<T> {
	args.get(index).and_then(|arg| arg.parse().ok())
}

fn get_user_input<T: FromStr, R: io::BufRead>(stdin: &mut R) -> T {
	loop {
		let mut buffer = String::new();
		stdin.read_line(&mut buffer).unwrap();
		match buffer.trim().parse() {
			Ok(value) => break value,
			Err(_) => println!("Invalid input. Please enter a valid value:")
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use std::io::Cursor;

	#[test]
	fn should_retrieve_plugin_version_by_app_arguments() {
		// GIVEN the app arguments
		// AND the user inputs
		let args = vec!["app", "1.21", "plugin"];
		let mut stdin = Cursor::new("\n");

		// WHEN we want to get the plugin version
		let plugin_version: f32 = get_parameter(&args, 1, &mut stdin);

		// THEN the version should be retruned
		assert_eq!(1.21, plugin_version);
	}

	#[test]
	fn should_retrieve_plugin_version_by_user_input_with_invalid_app_argument() {
		// GIVEN the app arguments with an invalid format for the version
		// AND the user inputs
		let args = vec!["app", "invalid", "plugin"];
		let mut stdin = Cursor::new("1.21\n");

		// WHEN we want to get the plugin version
		let plugin_version: f32 = get_parameter(&args, 1, &mut stdin);

		// THEN the plugin version should be retrieved from the user input
		assert_eq!(1.21, plugin_version);
	}

	#[test]
	fn should_retrieve_plugin_version_by_user_input_with_missing_app_argument() {
		// GIVEN the app arguments without the version
		// AND the user inputs
		let args = vec!["app"];
		let mut stdin = Cursor::new("1.21\n");

		// WHEN we want to get the plugin version
		let plugin_version: f32 = get_parameter(&args, 1, &mut stdin);

		// THEN the plugin version should be retrieved from the user input
		assert_eq!(1.21, plugin_version);
	}

	#[test]
	fn should_retrieve_plugin_version_by_user_input_with_invalid_user_input() {
		// GIVEN the app arguments without the version
		// AND the user inputs
		let args = vec!["app"];
		let mut stdin = Cursor::new("plugin_test\n1.21\n");

		// WHEN we want to get the plugin version
		let plugin_name: f32 = get_parameter(&args, 1, &mut stdin);

		// THEN the plugin version should be retrieved from the user input
		assert_eq!(1.21, plugin_name);
	}

	#[test]
	fn should_retrieve_plugin_name_by_app_arguments() {
		// GIVEN the app arguments
		// AND the user inputs
		let args = vec!["app", "1.21", "plugin_test"];
		let mut stdin = Cursor::new("\n");

		// WHEN we want to get the plugin version
		let plugin_name: String = get_parameter(&args, 2, &mut stdin);

		// THEN the version should be retruned
		assert_eq!(String::from("plugin_test"), plugin_name);
	}

	#[test]
	fn should_retrieve_plugin_name_by_user_input_with_invalid_app_argument() {
		// GIVEN the app arguments with an invalid format for the version
		// AND the user inputs
		let args = vec!["app", "invalid", "plugin_test"];
		let mut stdin = Cursor::new("plugin_test\n");

		// WHEN we want to get the plugin version
		let plugin_name: String = get_parameter(&args, 2, &mut stdin);

		// THEN the plugin version should be retrieved from the user input
		assert_eq!(String::from("plugin_test"), plugin_name);
	}

	#[test]
	fn should_retrieve_plugin_name_by_user_input_with_missing_app_argument() {
		// GIVEN the app arguments without the version
		// AND the user inputs
		let args = vec!["app"];
		let mut stdin = Cursor::new("plugin_test\n");

		// WHEN we want to get the plugin version
		let plugin_name: String = get_parameter(&args, 2, &mut stdin);

		// THEN the plugin version should be retrieved from the user input
		assert_eq!(String::from("plugin_test"), plugin_name);
	}
}