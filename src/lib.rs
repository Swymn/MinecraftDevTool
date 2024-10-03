mod parameter_reader;
mod project_generator;

pub use parameter_reader::get_parameters;
pub use project_generator::spigot::SpigotGenerator;
pub use project_generator::{get_project_type, ProjectGeneratorType};
