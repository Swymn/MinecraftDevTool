#[allow(dead_code)]
fn get_template_generator(type_str: &str) -> TemplateGeneratorType {
    match type_str {
        "spigot" => TemplateGeneratorType::Spigot(SpigotTemplateGenerator),
        _ => panic!("Unknown template generator type: {}", type_str),
    }
}

#[allow(dead_code)]
enum TemplateGeneratorType {
    Spigot(SpigotTemplateGenerator),
}

#[derive(Default)]
struct SpigotTemplateGenerator;

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
        assert!(matches!(result, TemplateGeneratorType::Spigot(_)));
    }
}
