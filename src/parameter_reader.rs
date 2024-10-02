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
    args: &[String],
    buffer_reader: &mut B,
    parameter_amount: usize,
    input: &'static str,
) -> Vec<String> {
    let mut parameters: Vec<String> = Vec::with_capacity(parameter_amount);
    for i in 0..parameter_amount {
        let parameter = args
            .get(i)
            .cloned()
            .unwrap_or_else(|| get_user_input(buffer_reader, input));
        parameters.push(parameter);
    }

    parameters
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup_test(args: Vec<String>, input: &str, parameter_amount: u8) -> Vec<String> {
        let mut buffer_reader = Cursor::new(input);
        get_parameters(&args, &mut buffer_reader, parameter_amount as usize, "")
    }

    #[test]
    fn get_parameters_should_return_parameter_from_one_args() {
        // GIVEN an args array with one value
        let args = vec![String::from("toto")];

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "\n", 1);

        // THEN the parameter returned should be 'toto'
        assert_eq!("toto", parameter.first().unwrap());
    }

    #[test]
    fn get_parameters_should_return_parameter_from_many_args() {
        // GIVEN an args array with multiple values
        let args = vec![
            String::from("toto"),
            String::from("tata"),
            String::from("titi"),
        ];

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "\n", 1);

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter.first().unwrap())
    }

    #[test]
    fn get_parameters_should_return_parameter_from_user_input_because_of_empty_args() {
        // GIVEN an empty args array
        let args: Vec<String> = Vec::new();

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "toto\n", 1);

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter.first().unwrap());
    }

    #[test]
    fn get_parameters_should_return_multiple_parameters_from_args() {
        // GIVEN an empty args array
        let args = vec![
            String::from("toto"),
            String::from("tata"),
            String::from("titi"),
        ];

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "\n", 2);

        // THEN the parameter should be 'toto, tata'
        assert_eq!(vec![String::from("toto"), String::from("tata")], parameter);
    }

    #[test]
    fn get_parameters_should_return_multiple_parameters_from_args_and_user_input() {
        // GIVEN an args array with one value
        let args = vec![String::from("toto")];

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "tata\n", 2);

        // THEN the parameter should be 'toto, tata'
        assert_eq!(vec![String::from("toto"), String::from("tata")], parameter);
    }

    #[test]
    fn get_parameters_should_return_multiple_parameters_from_user_input() {
        // GIVEN an empty args array
        let args = vec![];

        // WHEN we call the get_parameter function
        let parameter = setup_test(args, "toto\ntata\n", 2);

        // THEN the parameter should be 'toto, tata'
        assert_eq!(vec![String::from("toto"), String::from("tata")], parameter);
    }
}
