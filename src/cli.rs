use std::collections::VecDeque;
use std::io::BufRead;

pub struct Cli {
    parameters: VecDeque<String>,
    flags: VecDeque<String>,
    input_buffer: Box<dyn BufRead>,
}

impl Cli {
    pub fn new(args: Vec<String>, input_buffer: Box<dyn BufRead>) -> Self {
        let mut parameters = Vec::with_capacity(args.len());
        let mut flags = Vec::with_capacity(args.len());

        for arg in args {
            match arg {
                arg if arg.starts_with("--") => flags.push(arg),
                arg => parameters.push(arg),
            }
        }

        Self {
            parameters: VecDeque::from(parameters),
            flags: VecDeque::from(flags),
            input_buffer,
        }
    }

    pub fn get_next_parameter(&mut self, input: &'static str, mandatory: bool) -> Option<String> {
        let parameter = self.parameters.pop_front();
        if let Some(param) = parameter {
            Some(param)
        } else if mandatory {
            Some(self.get_user_input(input))
        } else {
            None
        }
    }

    pub fn with_server(&mut self) -> bool {
        if self.flags.contains(&String::from("--with-server")) {
            true
        } else {
            let user_input = self.get_user_input("Do you want to include a server? (yes/no)");
            user_input == "yes" || user_input == "y"
        }
    }

    pub fn get_user_input(&mut self, display_input: &'static str) -> String {
        let mut input = String::new();
        println!("{}", display_input);
        self.input_buffer.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn create_cli_instance(args: Vec<String>, user_input: &str) -> Cli {
        let input_buffer = Cursor::new(String::from(user_input));
        Cli::new(args, Box::new(input_buffer))
    }

    #[test]
    fn get_next_parameter_should_return_parameter_from_args() {
        // GIVEN an args array with one value
        let args = vec![String::from("toto")];
        let mut cli = create_cli_instance(args, "");

        // WHEN we call the get_next_parameter function
        let parameter = cli.get_next_parameter("", true);

        // THEN the parameter returned should be 'toto'
        assert_eq!("toto", parameter.unwrap());
    }

    #[test]
    fn get_next_parameter_should_return_parameter_from_user_input() {
        // GIVEN an empty args array
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "toto\n");

        // WHEN we call the get_next_parameter function
        let parameter = cli.get_next_parameter("", true);

        // THEN the parameter should be 'toto'
        assert_eq!("toto", parameter.unwrap());
    }

    #[test]
    fn get_next_parameter_should_return_none() {
        // GIVEN an empty args array
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "");

        // WHEN we call the get_next_parameter function
        let parameter = cli.get_next_parameter("", false);

        // THEN the parameter should be None
        assert_eq!(None, parameter);
    }

    #[test]
    fn with_server_should_return_true() {
        // GIVEN an args array with a flag
        let args = vec![String::from("--with-server")];
        let mut cli = create_cli_instance(args, "");

        // WHEN we call the with_server function
        let with_server = cli.with_server();

        // THEN the with_server should be true
        assert!(with_server);
    }

    #[test]
    fn with_server_should_return_false() {
        // GIVEN an args array without a flag
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "no\n");

        // WHEN we call the with_server function
        let with_server = cli.with_server();

        // THEN the with_server should be false
        assert!(!with_server);
    }

    #[test]
    fn with_server_should_return_true_from_user_input() {
        // GIVEN an args array without a flag
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "yes\n");

        // WHEN we call the with_server function
        let with_server = cli.with_server();

        // THEN the with_server should be true
        assert!(with_server);
    }

    #[test]
    fn with_server_should_return_true_from_user_input_shortcut() {
        // GIVEN an args array without a flag
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "y\n");

        // WHEN we call the with_server function
        let with_server = cli.with_server();

        // THEN the with_server should be true
        assert!(with_server);
    }

    #[test]
    fn get_user_input_should_return_user_input() {
        // GIVEN an empty args array
        let args: Vec<String> = Vec::new();
        let mut cli = create_cli_instance(args, "toto\n");

        // WHEN we call the get_user_input function
        let user_input = cli.get_user_input("Please provide a parameter:");

        // THEN the user_input should be 'toto'
        assert_eq!("toto", user_input);
    }
}
