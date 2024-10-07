fn generate_file_content(template: &str, name: &str, version: &str, group_id: &str) -> String {
    template
        .replace("{name}", name)
        .replace("{version}", version)
        .replace("{group_id}", group_id)
}

pub fn generate_pom_xml_content(name: &str, version: &str, group_id: &str) -> String {
    generate_file_content(
        r#"<?xml version="1.0" encoding="UTF-8"?>
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
        <spigot.version>{version}-R0.1-SNAPSHOT</spigot.version>
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
            <version>${spigot.version}</version>
            <scope>provided</scope>
        </dependency>
    </dependencies>
</project>
    "#,
        name,
        version,
        group_id,
    )
}

pub fn generate_main_java_content(name: &str, group_id: &str) -> String {
    generate_file_content(
        r#"package {group_id};

import org.bukkit.plugin.java.JavaPlugin;

public class {name} extends JavaPlugin {

    @Override
    public void onEnable() {
        getLogger().info("Hello, SpigotMC!");
    }

    @Override
    public void onDisable() {
        getLogger().info("Goodbye, SpigotMC!");
    }
}
    "#,
        name,
        "",
        group_id,
    )
}

pub fn generate_plugin_yml_content(name: &str, group_id: &str) -> String {
    generate_file_content(
        r#"name: {name}
version: 1.0
main: {group_id}.{name}
author: Notch # Set yours
    "#,
        name,
        "",
        group_id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_file_content() {
        let template = "Name: {name}, Version: {version}, Group ID: {group_id}";
        let name = "TestName";
        let version = "1.0.0";
        let group_id = "com.test";
        let result = generate_file_content(template, name, version, group_id);
        let expected = "Name: TestName, Version: 1.0.0, Group ID: com.test";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_pom_xml_content() {
        let name = "TestName";
        let version = "1.0.0";
        let group_id = "com.test";
        let result = generate_pom_xml_content(name, version, group_id);
        assert!(result.contains("<artifactId>TestName</artifactId>"));
        assert!(result.contains("<version>1.0.0</version>"));
        assert!(result.contains("<groupId>com.test</groupId>"));
    }

    #[test]
    fn test_generate_main_java_content() {
        let name = "TestName";
        let group_id = "com.test";
        let result = generate_main_java_content(name, group_id);
        assert!(result.contains("package com.test;"));
        assert!(result.contains("public class TestName extends JavaPlugin {"));
    }

    #[test]
    fn test_generate_plugin_yml_content() {
        let name = "TestName";
        let group_id = "com.test";
        let result = generate_plugin_yml_content(name, group_id);
        assert!(result.contains("name: TestName"));
        assert!(result.contains("main: com.test.TestName"));
    }
}
