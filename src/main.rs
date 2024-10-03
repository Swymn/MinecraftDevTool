use minecraft_dev_tool::{get_parameters, get_project_type, ProjectGeneratorType, SpigotGenerator};
use std::io::BufReader;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut input_buffer = BufReader::new(io::stdin());

    let project_type = get_parameters(&args, &mut input_buffer, "Please provide the project type:");

    let project_generator = get_project_type(&project_type);

    if let Some(project_generator) = project_generator {
        match project_generator {
            ProjectGeneratorType::Spigot => {
                let name = get_parameters(
                    &args,
                    &mut input_buffer,
                    "Please provide a name for your project",
                );
                let version = get_parameters(
                    &args,
                    &mut input_buffer,
                    "Please provide a version for your project",
                );
                let group_id = get_parameters(
                    &args,
                    &mut input_buffer,
                    "Please provide a group id for your project",
                );

                let spigot_generator = SpigotGenerator::new(name, version, group_id);

                spigot_generator.generate_project();
                println!("Project generated!");
            }
        }
    } else {
        eprintln!("Unable to determine the project generator");
    }
}
