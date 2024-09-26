use spigot::SpigotTemplateGenerator;
use template::TemplateGenerator;

mod spigot;
mod template;

pub fn get_template_generator(type_str: &str) -> Box<dyn TemplateGenerator> {
    match type_str {
        "spigot" => Box::new(SpigotTemplateGenerator::default()),
        _ => panic!("Unknown template generator type: {}", type_str),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_template_generator_return_template_default() {
        // GIVEN a a string type;
        let type_str = "spigot";

        // WHEN get_template_generator is called with the type_str;
        let result = get_template_generator(type_str);

        // THEN the result should be a SpigotTemplateGenerator;
        assert!(result
            .as_any()
            .downcast_ref::<SpigotTemplateGenerator>()
            .is_some());
    }
}
