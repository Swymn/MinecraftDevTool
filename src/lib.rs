mod parameter_reader;
mod project_generator;

use std::io::BufReader;
use std::{env, io};

use crate::parameter_reader::get_parameters;
use crate::project_generator::spigot::SpigotGenerator;
use crate::project_generator::{get_project_type, ProjectGeneratorType};

pub fn execute() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut input_buffer = BufReader::new(io::stdin());

    let project_type = get_parameters(
        &mut args,
        &mut input_buffer,
        "Please provide the project type:",
    );

    let project_generator = get_project_type(&project_type);

    if let Some(project_generator) = project_generator {
        match project_generator {
            ProjectGeneratorType::Spigot => {
                let name = get_parameters(
                    &mut args,
                    &mut input_buffer,
                    "Please provide a name for your project",
                );
                let version = get_parameters(
                    &mut args,
                    &mut input_buffer,
                    "Please provide a version for your project",
                );
                let group_id = get_parameters(
                    &mut args,
                    &mut input_buffer,
                    "Please provide a group id for your project",
                );
                let path = get_parameters(
                    &mut args,
                    &mut input_buffer,
                    "Please provide a path for your project, leave empty for current directory",
                );

                let spigot_generator = SpigotGenerator::new(name, version, group_id, path);

                spigot_generator.generate_project();
                println!("Project generated!");
            }
        }
    } else {
        eprintln!("Unable to determine the project generator");
    }
}
