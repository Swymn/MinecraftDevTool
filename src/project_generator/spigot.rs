use std::env;

use crate::errors::GeneratorError;
use crate::project_generator::content_generator::{
    generate_main_java_content, generate_plugin_yml_content, generate_pom_xml_content,
};
use crate::project_generator::file_operations::{create_directory, create_file};

pub struct SpigotGenerator {
    name: String,
    version: String,
    group_id: String,
    path: String,
}

impl SpigotGenerator {
    pub fn new(name: String, version: String, group_id: String, path: Option<String>) -> Self {
        Self {
            name: Self::format_name(name),
            version,
            group_id,
            path: Self::get_project_path(path),
        }
    }

    fn format_name(name: String) -> String {
        name.split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut chars = s.chars();
                chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
            })
            .collect()
    }

    fn get_project_path(path: Option<String>) -> String {
        match path {
            None => env::current_dir().unwrap().to_str().unwrap().to_string(),
            Some(path) => match path {
                path if path.is_empty() => {
                    env::current_dir().unwrap().to_str().unwrap().to_string()
                }
                path if path.starts_with("./") => {
                    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
                    format!("{}/{}", current_dir, path.trim_start_matches("./"))
                }
                path => path,
            },
        }
    }

    pub fn generate_project(&self) -> Result<(), GeneratorError> {
        let project_name = self.name.to_lowercase();
        let project_path = format!("{}/{}", self.path, project_name);
        create_directory(&project_path)?;

        create_file(
            &format!("{}/pom.xml", project_path),
            &generate_pom_xml_content(&self.name, &self.version, &self.group_id),
        )?;

        let resources_path = format!("{}/src/main/resources", project_path);
        create_directory(&resources_path)?;
        create_file(
            &format!("{}/plugin.yml", resources_path),
            &generate_plugin_yml_content(&self.name, &self.group_id),
        )?;

        let java_path = format!(
            "{}/src/main/java/{}",
            project_path,
            self.group_id.replace(".", "/")
        );
        create_directory(&java_path)?;
        create_file(
            &format!("{}/{}.java", java_path, self.name),
            &generate_main_java_content(&self.name, &self.group_id),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn clean_up(folder_name: &str) {
        fs::remove_dir_all(folder_name).expect("Unable to remove project folder");
    }
    #[test]
    fn generate_project_should_generate_pom_xml_file() {
        // GIVEN a spigot generator;
        let spigot_generator = SpigotGenerator::new(
            String::from("TestOne"),
            String::from("1.8.8"),
            String::from("com.test"),
            Some(String::from("./test")),
        );

        // WHEN we generate the project
        let result = spigot_generator.generate_project();

        // THEN the project folder should contain a pom.xml file
        assert!(result.is_ok(), "Project generation failed");

        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path_name = &format!("{}/{}", spigot_generator.path, name_in_lowercase);
        let project_path = Path::new(&project_path_name);
        assert!(
            project_path.exists() && project_path.is_dir(),
            "Project folder does not exist"
        );

        let pom_file_path = project_path.join("pom.xml");
        assert!(
            pom_file_path.exists() && pom_file_path.is_file(),
            "pom.xml file does not exist"
        );

        // AND the content of the pom.xml file should be the same as the one generated
        let pom_xml_content =
            fs::read_to_string(pom_file_path).expect("Unable to read pom.xml file");
        let expected_pom_xml_content = generate_pom_xml_content(
            &spigot_generator.name,
            &spigot_generator.version,
            &spigot_generator.group_id,
        );
        assert_eq!(pom_xml_content, expected_pom_xml_content);

        // Clean up
        clean_up(&project_path_name);
    }

    #[test]
    fn generate_project_should_generate_plugin_yml_file() {
        // GIVEN a spigot generator;
        let spigot_generator = SpigotGenerator::new(
            String::from("TestThree"),
            String::from("1.21"),
            String::from("com.test"),
            Some(String::from("./test")),
        );

        // WHEN we generate the project
        let result = spigot_generator.generate_project();

        // THEN the project folder should contain a main java file
        assert!(result.is_ok(), "Project generation failed");

        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path_name = &format!("{}/{}", spigot_generator.path, name_in_lowercase);
        let project_path = Path::new(&project_path_name);
        let plugin_yml_file_path = project_path.join("src/main/resources/plugin.yml");
        assert!(
            plugin_yml_file_path.exists() && plugin_yml_file_path.is_file(),
            "plugin.yml file does not exist"
        );

        // AND the content of the main java file should be the same as the one generated
        let plugin_yml_content =
            fs::read_to_string(plugin_yml_file_path).expect("Unable to read plugin.yml file");
        let expected_plugin_yml_content =
            generate_plugin_yml_content(&spigot_generator.name, &spigot_generator.group_id);
        assert_eq!(plugin_yml_content, expected_plugin_yml_content);

        // Clean up
        clean_up(&project_path_name);
    }

    #[test]
    fn generate_project_should_generate_main_java_file() {
        // GIVEN a spigot generator;
        let spigot_generator = SpigotGenerator::new(
            String::from("TestTwo"),
            String::from("1.21"),
            String::from("com.test"),
            Some(String::from("./test")),
        );

        // WHEN we generate the project
        let result = spigot_generator.generate_project();

        // THEN the project folder should contain a main java file
        assert!(result.is_ok(), "Project generation failed");

        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path_name = &format!("{}/{}", spigot_generator.path, name_in_lowercase);
        let project_path = Path::new(&project_path_name);
        let main_java_file_path = project_path.join("src/main/java/com/test/TestTwo.java");
        assert!(
            main_java_file_path.exists() && main_java_file_path.is_file(),
            "Main java file does not exist"
        );

        // AND the content of the main java file should be the same as the one generated
        let main_java_content =
            fs::read_to_string(main_java_file_path).expect("Unable to read main java file");
        let expected_main_java_content =
            generate_main_java_content(&spigot_generator.name, &spigot_generator.group_id);
        assert_eq!(main_java_content, expected_main_java_content);

        // Clean up
        clean_up(&project_path_name);
    }

    #[test]
    fn parse_name_should_parse_into_pascal_case() {
        // GIVEN a name
        let name = String::from("ezezz-ezfze_zefze=ff:pofkj");

        // WHEN we parsed the name
        let parsed_name = SpigotGenerator::format_name(name);

        // THEN the name should look like so
        assert_eq!("EzezzEzfzeZefzeFfPofkj".to_string(), parsed_name)
    }

    #[test]
    fn get_project_path_should_return_test_folder_path_from_relative_path() {
        // GIVEN a path
        let path = Some(String::from("./test"));

        // WHEN we get the path
        let path = SpigotGenerator::get_project_path(path);

        // THEN the path should be this
        assert_eq!(
            format!("{}/test", env::current_dir().unwrap().to_str().unwrap()),
            path
        );
    }

    #[test]
    fn get_project_path_should_return_test_folder_path_from_absolute_path() {
        // GIVEN a path
        let path = Some(String::from("/test"));

        // WHEN we get the path
        let path = SpigotGenerator::get_project_path(path);

        // THEN the path should be this
        assert_eq!("/test", path);
    }

    #[test]
    fn get_project_path_should_return_absolute_path_empty_path() {
        // GIVEN a path
        let path = Some(String::from(""));

        // WHEN we get the path
        let path = SpigotGenerator::get_project_path(path);

        // THEN the path should be this
        assert_eq!(
            format!("{}", env::current_dir().unwrap().to_str().unwrap()),
            path
        );
    }

    #[test]
    fn get_project_path_should_return_absolute_path_none() {
        // GIVEN a path
        let path = None;

        // WHEN we get the path
        let path = SpigotGenerator::get_project_path(path);

        // THEN the path should be this
        assert_eq!(
            format!("{}", env::current_dir().unwrap().to_str().unwrap()),
            path
        );
    }
}
