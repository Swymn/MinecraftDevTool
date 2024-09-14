# Minecraft Dev Tools

The main goal for this tool is to provide a way to easily create template for Minecraft plugins, mods, etc...
Here are the diagrams that show how the tool operates:

```mermaid
sequenceDiagram
	participant User
	participant Tool

	User->>Tool: Create a new project with somes informations
	Tool->>Tool: Get the template generator for the project type
	Tool->>Tool: Get the parameters reader for the project type
	Tool->>Tool: Read the parameters
	Tool->>Tool: Generate the template with the parameters
	Tool->>User: Project created according to the informations
```

## Global Architecture

```mermaid
---
Tool Structure
---

classDiagram
	class Tool {
		+getTemplateGenerator(String type) TemplateGeneratorType

		+createProject(TemplateGenerator generator)
	}

	class TemplateParameterReader {
		<<interface>>
		+readParameters() Map~String, String~
	}

	class TemplateGenerator {
		<<interface>>
		+generateTemplate(Map~String, String~)
	}

	class SpigotPluginParameterReader {
		+readParameters() Map~String, String~
	}

	class SpigotPluginGenerator {
		+generateTemplate()
	}

	class TemplateGeneratorType {
		<<enum>>
		SpiotPlugin -> SpigotPluginGenerator
	}

	TemplateGenerator <|-- SpigotPluginGenerator
	TemplateParameterReader <|-- SpigotPluginParameterReader
	SpigotPluginGenerator <|-- TemplateGeneratorType : uses
	TemplateGeneratorType <|.. Tool : uses
	SpigotPluginParameterReader <.. SpigotPluginGenerator : uses

```