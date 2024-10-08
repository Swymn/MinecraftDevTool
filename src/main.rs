use env_logger::{Builder, Target};
use minecraft_dev_tool::run;

fn main() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);

    builder.init();

    match run() {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("{}", error),
    }
}
