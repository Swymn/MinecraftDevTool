#[cfg(test)]
use super::super::*;

#[test]
fn get_project_generator_type_should_return_spigot() {
    // GIVEN a string that represent a project type
    let project_generator_type = String::from("spigot");

    // WHEN we parse it
    let project_generator = get_project_type(&project_generator_type);

    // THEN the project type might an instance of spigot
    assert!(project_generator.is_some());
    assert!(matches!(
        project_generator,
        Some(ProjectGeneratorType::Spigot)
    ));
}

#[test]
fn get_project_generator_type_should_return_none_for_empty_string() {
    // GIVEN an empty string
    let project_generator_type = String::from("");

    // WHEN we parse it
    let project_generator = get_project_type(&project_generator_type);

    // THEN the function return none
    assert!(project_generator.is_none());
}

#[test]
fn get_project_generator_type_should_spigot_for_unconventional_case() {
    // GIVEN an unconventional string
    let project_generator_type = String::from("SpIgOt");

    // WHEN we parse it
    let project_generator = get_project_type(&project_generator_type);

    // THEN the project type might an instance of spigot
    assert!(project_generator.is_some());
    assert!(matches!(
        project_generator,
        Some(ProjectGeneratorType::Spigot)
    ));
}
