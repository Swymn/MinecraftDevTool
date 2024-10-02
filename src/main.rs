use minecraft_dev_tool::{get_parameters, get_project_type, ProjectGeneratorType};
use std::io::BufReader;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut input_buffer = BufReader::new(io::stdin());

    let project_type = get_parameters(
        &args,
        &mut input_buffer,
        1,
        "Please provide the project type:",
    );
    let project_type = project_type.first().expect("No project type provided");

    let project_generator = get_project_type(project_type);

    if let Some(project_generator) = project_generator {
        match project_generator {
            ProjectGeneratorType::Spigot(_) => {
                println!("Spigot project generator selected");
            }
        }
    } else {
        eprintln!("Unable to determine the project generator");
    }
}
