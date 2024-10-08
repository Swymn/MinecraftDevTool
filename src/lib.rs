mod cli;
mod errors;
mod project_generator;

use crate::cli::Cli;
use crate::errors::GeneratorError;
use crate::project_generator::spigot::SpigotGenerator;
use crate::project_generator::{get_project_type, ProjectGeneratorType};
use log::info;
use std::io::BufReader;
use std::{env, io};

pub fn run<'a>() -> Result<&'a str, GeneratorError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_buffer = BufReader::new(io::stdin());
    let mut cli = Cli::new(args, Box::new(input_buffer));

    let project_type = cli
        .get_next_parameter(
            "Please provide the type of project you want to generate (spigot)",
            true,
        )
        .ok_or(GeneratorError::UnableToDetermineProjectGenerator)?;

    let project_generator = get_project_type(&project_type);

    if let Some(project_generator) = project_generator {
        match project_generator {
            ProjectGeneratorType::Spigot => {
                let name = cli
                    .get_next_parameter("Please provide a name for your project", true)
                    .ok_or(GeneratorError::UnableToReadMandatoryParameter)?;

                let version = cli
                    .get_next_parameter("Please provide a version for your project", true)
                    .ok_or(GeneratorError::UnableToReadMandatoryParameter)?;

                let group_id = cli
                    .get_next_parameter("Please provide a group id for your project", true)
                    .ok_or(GeneratorError::UnableToReadMandatoryParameter)?;

                let path = cli.get_next_parameter(
                    "Please provide a path for your project, leave empty for current directory",
                    false,
                );

                let spigot_generator = SpigotGenerator::new(name, version, group_id, path);
                let with_server = cli.with_server();

                info!("Creating project...");
                spigot_generator.generate_project()?;

                info!("with_server: {}", with_server);
                Ok("Project generated!")
            }
        }
    } else {
        Err(GeneratorError::UnableToDetermineProjectGenerator)
    }
}
