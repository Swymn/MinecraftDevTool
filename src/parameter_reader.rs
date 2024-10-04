use std::io;

fn get_user_input<B: io::BufRead>(buffer_reader: &mut B, input: &'static str) -> String {
    println!("{}", input);
    let mut user_input = String::new();
    buffer_reader
        .read_line(&mut user_input)
        .expect("Something went wrong while reading the input.");
    user_input.trim().to_string()
}

pub fn get_parameters<B: io::BufRead>(
    args: &mut Vec<String>,
    buffer_reader: &mut B,
    input: &'static str,
) -> String {
    if !args.is_empty() {
        args.remove(0)
    } else {
        get_user_input(buffer_reader, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup_test(args: &mut Vec<String>, input: &str) -> String {
        let mut buffer_reader = Cursor::new(input);
        get_parameters(args, &mut buffer_reader, "")
    }

    #[test]
    fn get_parameters_should_return_parameter_from_one_args() {
        // GIVEN an args array with one value
        let mut args = vec![String::from("toto")];

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "\n");

        // THEN the parameter returned should be 'toto'
        assert_eq!("toto", parameter);
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
        let parameter = setup_test(&mut args, "\n");

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter)
    }

    #[test]
    fn get_parameters_should_return_parameter_from_user_input_because_of_empty_args() {
        // GIVEN an empty args array
        let mut args: Vec<String> = Vec::new();

        // WHEN we call the get_parameter function
        let parameter = setup_test(&mut args, "toto\n");

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter);
    }
}
