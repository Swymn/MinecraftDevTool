use std::fs;

#[derive(Debug, PartialEq)]
pub struct SpigotGenerator {
    name: String,
    version: String,
    group_id: String,
}

impl SpigotGenerator {
    pub fn new(name: String, version: String, group_id: String) -> Self {
        Self {
            name,
            version: Self::get_valid_version(&version),
            group_id,
        }
    }

    fn get_valid_version(version: &str) -> String {
        String::from(match version {
            "1.4.6" => "1.4.6-R0.4-SNAPSHOT",
            "1.4.7" => "1.4.7-R1.1-SNAPSHOT",
            "1.5.1" => "1.5.1-R0.1-SNAPSHOT",
            "1.5.2" => "1.5.2-R1.1-SNAPSHOT",
            "1.6.2" => "1.6.2-R1.1-SNAPSHOT",
            "1.6.4" => "1.6.4-R2.1-SNAPSHOT",
            "1.7.2" => "1.7.2-R0.4-SNAPSHOT-1339",
            "1.7.5" => "1.7.5-R0.1-SNAPSHOT-1387",
            "1.7.8" => "1.7.8-R0.1-SNAPSHOT",
            "1.7.9" => "1.7.9-R0.2-SNAPSHOT",
            "1.7.10" => "1.7.10-SNAPSHOT-b1657",
            "1.8" => "1.8-R0.1-SNAPSHOT-latest",
            "1.8.3" => "1.8.3-R0.1-SNAPSHOT-latest",
            "1.8.4" => "1.8.4-R0.1-SNAPSHOT-latest",
            "1.8.5" => "1.8.5-R0.1-SNAPSHOT-latest",
            "1.8.6" => "1.8.6-R0.1-SNAPSHOT-latest",
            "1.8.7" => "1.8.7-R0.1-SNAPSHOT-latest",
            "1.8.8" => "1.8.8-R0.1-SNAPSHOT-latest",
            "1.9" => "1.9-R0.1-SNAPSHOT-latest",
            "1.9.2" => "1.9.2-R0.1-SNAPSHOT-latest",
            "1.9.4" => "1.9.4-R0.1-SNAPSHOT-latest",
            "1.10" => "1.10-R0.1-SNAPSHOT-latest",
            "1.10.2" => "1.10.2-R0.1-SNAPSHOT-latest",
            _ => version,
        })
    }

    fn generate_file_content(&self, template: &str) -> String {
        template
            .replace("{name}", &self.name)
            .replace("{version}", &self.version)
            .replace("{group_id}", &self.group_id)
    }

    fn generate_pom_xml_content(&self) -> String {
        self.generate_file_content(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd"
    xmlns="http://maven.apache.org/POM/4.0.0"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <modelVersion>4.0.0</modelVersion>
    <groupId>{group_id}</groupId>
    <artifactId>{name}</artifactId>
    <version>1.0.0</version>
    <packaging>jar</packaging>
    <name>{name}</name>
    <description>Test project</description>
    <properties>
        <maven.compiler.target>21</maven.compiler.target>
        <maven.compiler.source>21</maven.compiler.source>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
        <spigot.version>{version}</spigot.version>
    </properties>
    <repositories>
        <repository>
            <id>spigot-repo</id>
            <url>https://hub.spigotmc.org/nexus/content/repositories/snapshots/</url>
        </repository>
    </repositories>
    <dependencies>
        <dependency>
            <groupId>org.spigotmc</groupId>
            <artifactId>spigot-api</artifactId>
            <version>${{spigot.version}}</version>
            <scope>provided</scope>
        </dependency>
    </dependencies>
</project>
        "#)
    }

    fn generate_main_java_content(&self) -> String {
        self.generate_file_content(
            r#"package {group_id};

import org.bukkit.plugin.java.JavaPlugin;

public class {name} extends JavaPlugin {{

    @Override
    public void onEnable() {{
        getLogger().info("Hello, SpigotMC!");
    }}

    @Override
    public void onDisable() {{
        getLogger().info("Goodbye, SpigotMC!");
    }}
}}
        "#,
        )
    }

    fn generate_plugin_yml_content(&self) -> String {
        self.generate_file_content(
            r#"name: {name}
version: 1.0
main: {group_id}.{name}
author: Notch # Set yours
        "#,
        )
    }

    fn create_file(&self, path: &str, content: &str) {
        fs::write(path, content).unwrap_or_else(|_| panic!("Unable to create file at {}", path));
    }

    fn create_directory(&self, path: &str) {
        fs::create_dir_all(path)
            .unwrap_or_else(|_| panic!("Unable to create directory at {}", path));
    }

    pub fn generate_project(&self) {
        // Create project dir
        let project_name = self.name.to_lowercase();
        self.create_directory(&project_name);

        // Create pom.xml file
        self.create_file(
            &format!("{}/pom.xml", project_name),
            &self.generate_pom_xml_content(),
        );

        // Create plugin.yml file
        let resources_path = format!("{}/src/main/resources", project_name);
        self.create_directory(&resources_path);
        self.create_file(
            &format!("{}/plugin.yml", resources_path),
            &self.generate_plugin_yml_content(),
        );

        // Create main java file
        let java_path = format!(
            "{}/src/main/java/{}",
            project_name,
            self.group_id.replace(".", "/")
        );
        self.create_directory(&java_path);
        self.create_file(
            &format!("{}/{}.java", java_path, self.name),
            &self.generate_main_java_content(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        );

        // WHEN we generate the project
        spigot_generator.generate_project();

        // THEN the project folder should contain a pom.xml file
        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path = Path::new(&name_in_lowercase);
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
        let expected_pom_xml_content = spigot_generator.generate_pom_xml_content();
        assert_eq!(pom_xml_content, expected_pom_xml_content);

        // Clean up
        clean_up(&name_in_lowercase);
    }

    #[test]
    fn generate_project_should_generate_plugin_yml_file() {
        // GIVEN a spigot generator;
        let spigot_generator = SpigotGenerator::new(
            String::from("TestThree"),
            String::from("1.21"),
            String::from("com.test"),
        );

        // WHEN we generate the project
        spigot_generator.generate_project();

        // THEN the project folder should contain a main java file
        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path = Path::new(&name_in_lowercase);
        let plugin_yml_file_path = project_path.join("src/main/resources/plugin.yml");
        assert!(
            plugin_yml_file_path.exists() && plugin_yml_file_path.is_file(),
            "plugin.yml file does not exist"
        );

        // AND the content of the main java file should be the same as the one generated
        let plugin_yml_content =
            fs::read_to_string(plugin_yml_file_path).expect("Unable to read plugin.yml file");
        let expected_plugin_yml_content = spigot_generator.generate_plugin_yml_content();
        assert_eq!(plugin_yml_content, expected_plugin_yml_content);

        // Clean up
        clean_up(&name_in_lowercase);
    }

    #[test]
    fn generate_project_should_generate_main_java_file() {
        // GIVEN a spigot generator;
        let spigot_generator = SpigotGenerator::new(
            String::from("TestTwo"),
            String::from("1.21"),
            String::from("com.test"),
        );

        // WHEN we generate the project
        spigot_generator.generate_project();

        // THEN the project folder should contain a main java file
        let name_in_lowercase = spigot_generator.name.to_lowercase();
        let project_path = Path::new(&name_in_lowercase);
        let main_java_file_path = project_path.join("src/main/java/com/test/TestTwo.java");
        assert!(
            main_java_file_path.exists() && main_java_file_path.is_file(),
            "Main java file does not exist"
        );

        // AND the content of the main java file should be the same as the one generated
        let main_java_content =
            fs::read_to_string(main_java_file_path).expect("Unable to read main java file");
        let expected_main_java_content = spigot_generator.generate_main_java_content();
        assert_eq!(main_java_content, expected_main_java_content);

        // Clean up
        clean_up(&name_in_lowercase);
    }

    #[test]
    fn get_valid_version_should_return_valid_old_version() {
        // GIVEN a version
        let version = String::from("1.4.6");

        // WHEN we get the valid version
        let valid_version = SpigotGenerator::get_valid_version(&version);

        // THEN the version should be 1.4.6-R0.4-SNAPSHOT
        assert_eq!("1.4.6-R0.4-SNAPSHOT", valid_version);
    }
}
