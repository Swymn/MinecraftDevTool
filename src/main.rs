use minecraft_dev_tool::{get_project_type, ProjectGeneratorType};

fn main() {
	const GENERATOR: &str = "spigot";
	let project_generator = get_project_type(GENERATOR);
	
	if let Some(project_generator) = project_generator {
		match project_generator {
			ProjectGeneratorType::Spigot(_) => {
				println!("Spigot project generator selected");
			}
		}
	} else {
		eprintln!("Unable to determine the project generator");
	}
}