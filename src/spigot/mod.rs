use std::{
    any::Any,
    io::{self, Error},
};

use crate::template::{TemplateGenerator, TemplateParameterReader};

#[derive(Default, Debug, PartialEq)]
pub struct SpigotTemplateGenerator {
    project_name: String,
    version: String,
    group_id: String,
}

impl SpigotTemplateGenerator {
    fn ask_user<R: io::BufRead>(input_reader: &mut R) -> String {
        let mut input = String::new();
        input_reader.read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

impl TemplateGenerator for SpigotTemplateGenerator {
    fn generate(&self) -> Result<String, Error> {
        todo!();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TemplateParameterReader for SpigotTemplateGenerator {
    fn read_parameters<R: io::BufRead>(&mut self, args: Vec<String>, input_buffer: &mut R) {
        for (index, arg) in args.iter().enumerate() {
            match index {
                1 => self.project_name = arg.to_string(),
                2 => self.version = arg.to_string(),
                3 => self.group_id = arg.to_string(),
                _ => (),
            }
        }

        if self.project_name.is_empty() {
            println!("Enter the project name:");
            self.project_name = SpigotTemplateGenerator::ask_user(input_buffer);
        }

        if self.version.is_empty() {
            println!("Enter the version:");
            self.version = SpigotTemplateGenerator::ask_user(input_buffer);
        }

        if self.group_id.is_empty() {
            println!("Enter the group id:");
            self.group_id = SpigotTemplateGenerator::ask_user(input_buffer);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn should_read_parameters_from_program_arguments() {
        // GIVEN an fake program arguments
        let args = vec![
            "minecraft-dev-tool".to_string(),
            "project_name".to_string(),
            "1.20".to_string(),
            "com.github.minecraft-dev-tool".to_string(),
        ];
        // AND an input buffer
        let mut buffer = Cursor::new("\n");
        // AND a SpigotTemplate
        let mut spigot_template = SpigotTemplateGenerator::default();

        // WHEN reading parameters
        spigot_template.read_parameters(args, &mut buffer);

        // THEN the function should set the parameters into the template
        assert_eq!(spigot_template.project_name, "project_name".to_string());
        assert_eq!(spigot_template.version, "1.20".to_string());
        assert_eq!(
            spigot_template.group_id,
            "com.github.minecraft-dev-tool".to_string()
        );
    }

    #[test]
    fn should_return_error_when_parameters_are_not_provided() {
        // GIVEN an fake program arguments
        let args = vec!["minecraft-dev-tool".to_string()];
        // AND an input buffer
        let mut buffer = Cursor::new("project_name\n1.20\ncom.github.minecraft-dev-tool\n");
        // AND a SpigotTemplate
        let mut spigot_template = SpigotTemplateGenerator::default();

        // WHEN reading parameters
        spigot_template.read_parameters(args, &mut buffer);

        // THEN the function should return an error
        assert_eq!(spigot_template.project_name, "project_name".to_string());
        assert_eq!(spigot_template.version, "1.20".to_string());
        assert_eq!(
            spigot_template.group_id,
            "com.github.minecraft-dev-tool".to_string()
        );
    }
}
