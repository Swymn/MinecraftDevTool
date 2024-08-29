pub const POM_XML_FILE_CONTENT: fn(&str) -> String = |project_name| {
    format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
		 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
		 xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
	<modelVersion>4.0.0</modelVersion>

	<groupId>com.example</groupId>
	<artifactId>{}</artifactId>
	<version>1.0-SNAPSHOT</version>

	<properties>
		<project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
		<maven.compiler.source>1.8</maven.compiler.source>
		<maven.compiler.target>1.8</maven.compiler.target>
	</properties>

	<dependencies>
		<dependency>
			<groupId>org.bukkit</groupId>
			<artifactId>bukkit</artifactId>
			<version>1.16.5-R0.1-SNAPSHOT</version>
			<scope>provided</scope>
		</dependency>
	</dependencies>
</project>"#,
        project_name
    )
};

pub const PLUGN_YML_FILE_CONTENT: fn(&str) -> String = |project_name| {
    format!(
        r#"
name: {}
version: 1.0
main: plugin.{}
api-version: 1.16
author: JohnDoe"#,
        project_name, project_name
    )
};
