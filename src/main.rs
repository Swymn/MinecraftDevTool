use minecraft_dev_tool::get_template_generator;

fn main() {
    let template_generator = get_template_generator("spigot");
    let result = template_generator.generate();
    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("{}", e),
    }
}
