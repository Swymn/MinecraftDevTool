use minecraft_dev_tool::execute;

fn main() {
    match execute() {
        Ok(message) => println!("{}", message),
        Err(error) => eprintln!("{}", error),
    }
}
