mod spigot;
mod tests;

use spigot::SpigotGenerator;

pub fn get_project_type(project_generator_type: &str) -> Option<ProjectGeneratorType> {
    match project_generator_type.to_lowercase().as_str() {
        "spigot" => Some(ProjectGeneratorType::Spigot(SpigotGenerator)),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub enum ProjectGeneratorType {
    Spigot(SpigotGenerator),
}
