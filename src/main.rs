use minecraft_dev_tool::run;

fn main() {
    match run() {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("{}", error),
    }
}
