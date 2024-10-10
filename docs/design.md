# Minecraft Dev Tools

The main goal for this tool is to provide a way to easily create template for Minecraft plugins, mods, etc...
Here are the diagrams that show how the tool operates:

## Global Architecture


```mermaid
	classDiagram
    class MinecraftDevTools {
        +run() Result<\String, GeneratorError>
    }
    
    class GeneratorError {
        <<enum>>
        UnableToDetermineProjectGenerator,
        UnableToReadMandatoryParameter,
        FileCreationError,
        DirectoryCreationError,
    }
    
    class Cli {
        +get_next_parameter(input: &'static str, mandatory: bool) Option<\String\>
        +with_server() bool
    }
    
    class ProjectGenerator {
        +get_project_type(name: String) Option<\ProjectGenertorType\>
    }
    
    class ContentGenerator {
        +generate_pom_xml_content(name: &str, version: &str, group_id: &str) -> String
        +generate_main_java_content(name: &str, group_id: &str) -> String
        +generate_plugin_yml_content(name: &str, group_id: &str) -> String
    }
    
    class FileOperation {
        +create_file(path: &str, content: &str) Result<(), GeneratorError>
        +create_directory(path: &str) Result<(), GeneratorError>
    }
    
    class SpigotGenerator {
        +new(name: String, version: String, group_id: String, path: Option<String>) -> Self
        +generate_project(&self) -> Result<(), GeneratorError>
    }
    
    class ProjectGeneratorType {
        <<enum>>
        Spigot,
    }
	
	MinecraftDevTools --> ProjectGenerator : Use
	MinecraftDevTools --> Cli : Use
    ProjectGenerator --> ProjectGeneratorType : Use
    ProjectGeneratorType --> SpigotGenerator : Call
    SpigotGenerator --> ContentGenerator : Use
    SpigotGenerator --> FileOperation : Use
    
    FileOperation --> GeneratorError : Generate
```