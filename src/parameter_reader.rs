use std::io;

fn get_user_input<B: io::BufRead>(buffer_reader: &mut B, input: &'static str) -> String {
    println!("{}", input);
    let mut user_input = String::new();
    buffer_reader
        .read_line(&mut user_input)
        .expect("Something went wrong while reading the input.");
    user_input.trim().to_string()
}

pub fn get_parameter<B: io::BufRead>(
    args: &mut Vec<String>,
    buffer_reader: &mut B,
    input: &'static str,
    mandatory: bool,
) -> Option<String> {
    if !args.is_empty() {
        Some(args.remove(0))
    } else if mandatory {
        Some(get_user_input(buffer_reader, input))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup_test(args: &mut Vec<String>, input: &str, mandatory: bool) -> Option<String> {
        let mut buffer_reader = Cursor::new(input);
        get_parameter(args, &mut buffer_reader, "", mandatory)
    }

    #[test]
    fn get_parameters_should_return_parameter_from_one_args() {
        // GIVEN an args array with one value
        let mut args = vec![String::from("toto")];

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "\n", true);

        // THEN the parameter returned should be 'toto'
        assert_eq!("toto", parameter.unwrap());
    }

    #[test]
    fn get_parameters_should_return_parameter_from_many_args() {
        // GIVEN an args array with multiple values
        let mut args = vec![
            String::from("toto"),
            String::from("tata"),
            String::from("titi"),
        ];

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "\n", true);

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter.unwrap());
    }

    #[test]
    fn get_parameters_should_return_parameter_from_user_input_because_of_empty_args() {
        // GIVEN an empty args array
        let mut args: Vec<String> = Vec::new();

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "toto\n", true);

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter.unwrap());
    }

    #[test]
    fn get_parameters_should_return_empty_string_parameter() {
        // GIVEN an empty string array
        let mut args: Vec<String> = Vec::new();

        // WHEN we call the get parameter function
        let parameter = setup_test(&mut args, "\n", true);

        // THEN the parameter should be an empty string
        assert_eq!("", parameter.unwrap());
    }

    #[test]
    fn get_parameters_should_return_none_because_of_empty_args_and_not_mandatory() {
        // GIVEN an empty args array
        let mut args: Vec<String> = Vec::new();

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "toto\n", false);

        // THEN the parameter should be None
        assert_eq!(None, parameter);
    }

    #[test]
    fn get_parameters_should_return_none_because_of_empty_args_and_empty_buffer_and_not_mandatory()
    {
        // GIVEN an empty args array
        let mut args: Vec<String> = Vec::new();

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "\n", false);

        // THEN the parameter should be None
        assert_eq!(None, parameter);
    }

    #[test]
    fn get_parameters_should_return_some_because_of_non_empty_args_and_not_mandatory() {
        // GIVEN an empty args array
        let mut args: Vec<String> = vec![String::from("toto")];

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "\n", false);

        // THEN the parameter should be None
        assert!(parameter.is_some());
        assert_eq!(Some(String::from("toto")), parameter);
    }
}
