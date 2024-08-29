use std::path::Path;

const PROJECT_FOLDER: &str = "minecraft-plugins";
const FORMATTED_PROJECT_NAME: fn(&str) -> String = |project_name| project_name.replace(" ", "-");

fn _to_pascal_case(project_name: &str) -> String {
    project_name
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn create_project_structure(project_name: &str) -> Result<(), String> {
    let project_name = FORMATTED_PROJECT_NAME(project_name);

    let project_folder = format!("{}/{}", PROJECT_FOLDER, project_name);
    let src_folder = format!("{}/src", project_folder);
    let main_folder = format!("{}/main", src_folder);
    let resources_folder = format!("{}/resources", main_folder);
    let java_folder = format!("{}/java", main_folder);

    if Path::new(&project_folder).exists() {
        return Err(format!(
            "The project folder {} already exists",
            project_folder
        ));
    }

    std::fs::create_dir_all(resources_folder).unwrap();
    std::fs::create_dir_all(java_folder).unwrap();

    Ok(())
}

fn create_project_files(project_name: &str) -> Result<(), String> {
    let project_name = FORMATTED_PROJECT_NAME(project_name);

    let pom_file = format!("{}/{}/pom.xml", PROJECT_FOLDER, project_name);
    let plugin_yml_file = format!(
        "{}/{}/src/main/resources/plugin.yml",
        PROJECT_FOLDER, project_name
    );
    let main_class_file = format!(
        "{}/{}/src/main/java/plugin/{}.java",
        PROJECT_FOLDER,
        project_name,
        _to_pascal_case(&project_name)
    );

    let pom_content = "";

    let plugin_yml_content = "";

    let main_class_content = "";

    std::fs::write(pom_file, pom_content).unwrap();
    std::fs::write(plugin_yml_file, plugin_yml_content).unwrap();
    std::fs::write(main_class_file, main_class_content).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::Path;

    fn remove_folders() {
        let _ = std::fs::remove_dir_all(PROJECT_FOLDER);
    }

    #[test]
    fn should_create_project_folders() {
        // GIVEN a project name
        let project_name = "test";

        // WHEN we create the project folders
        let result = create_project_structure(project_name);

        // THEN the project folders should be created
        assert!(result.is_ok());
        assert!(Path::new(&format!("{}/{}", PROJECT_FOLDER, project_name)).exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/resources",
            PROJECT_FOLDER, project_name
        ))
        .exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/java",
            PROJECT_FOLDER, project_name
        ))
        .exists());

        // AND THEN remove the project folders
        remove_folders();
    }

    #[test]
    fn should_create_project_folders_with_not_formated_name() {
        // GIVEN a project name
        let project_name = "test project";

        // WHEN we create the project folders
        let result = create_project_structure(project_name);

        // THEN the project folders should be created
        let project_name = FORMATTED_PROJECT_NAME(project_name);
        assert!(result.is_ok());
        assert!(Path::new(&format!("{}/{}", PROJECT_FOLDER, project_name)).exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/resources",
            PROJECT_FOLDER, project_name
        ))
        .exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/java",
            PROJECT_FOLDER, project_name
        ))
        .exists());

        // AND THEN remove the project folders
        remove_folders();
    }

    #[test]
    fn should_not_create_project_because_of_existing_folder() {
        // GIVEN a project name
        let project_name = "test";

        // WHEN we create the project folders
        let result = create_project_structure(project_name);

        // THEN the project folders should be created
        assert!(result.is_ok());
        assert!(Path::new(&format!("{}/{}", PROJECT_FOLDER, project_name)).exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/resources",
            PROJECT_FOLDER, project_name
        ))
        .exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/java",
            PROJECT_FOLDER, project_name
        ))
        .exists());

        // WHEN we create the project folders again
        let result = create_project_structure(project_name);

        // THEN the project folders should not be created
        assert!(result.is_err());
        assert!(Path::new(&format!("{}/{}", PROJECT_FOLDER, project_name)).exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/resources",
            PROJECT_FOLDER, project_name
        ))
        .exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/java",
            PROJECT_FOLDER, project_name
        ))
        .exists());

        // AND THEN remove the project folders
        remove_folders();
    }

    #[test]
    fn should_parse_string_into_camel_case() {
        // GIVEN a string
        let project_name = "test project";

        // WHEN we parse the string into camel case
        let camel_case = _to_pascal_case(project_name);

        // THEN the string should be in camel case
        assert_eq!("TestProject", camel_case);
    }

    #[test]
    fn should_create_all_the_projects_file() {
        // GIVEN a project name
        let project_name = "test";

        // WHEN we create the project folders
        let folders_result = create_project_structure(project_name);
        // AND we create the files inside the project
        let files_result = create_project_files(project_name);

        // THEN both results are OK
        assert!(folders_result.is_ok());
        assert!(files_result.is_ok());
        // AND the following files should be created
        assert!(Path::new(&format!("{}/{}/pom.xml", PROJECT_FOLDER, project_name)).exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/resources/plugin.yml",
            PROJECT_FOLDER, project_name
        ))
        .exists());
        assert!(Path::new(&format!(
            "{}/{}/src/main/java/plugin/{}.java",
            PROJECT_FOLDER,
            project_name,
            _to_pascal_case(project_name)
        ))
        .exists());
    }
}
