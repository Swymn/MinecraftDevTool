#[derive(Debug)]
#[allow(dead_code)]
pub struct ApplicationParameter {
    version: String,
    plugin_name: String,
    group_id: String,
}

#[derive(Default)]
pub struct DefaultApplicationParameterBuilder {
    version: Option<String>,
    plugin_name: Option<String>,
    group_id: Option<String>,
}

pub trait ApplicationParameterBuilder {
    type OutputType;

    fn set_version(&mut self, version: String) -> &mut Self;
    fn set_plugin_name(&mut self, plugin_name: String) -> &mut Self;
    fn set_group_id(&mut self, group_id: String) -> &mut Self;

    fn build(&mut self) -> Self::OutputType;
}

impl ApplicationParameterBuilder for DefaultApplicationParameterBuilder {
    type OutputType = ApplicationParameter;

    fn set_version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);
        self
    }

    fn set_plugin_name(&mut self, plugin_name: String) -> &mut Self {
        self.plugin_name = Some(plugin_name);
        self
    }

    fn set_group_id(&mut self, group_id: String) -> &mut Self {
        self.group_id = Some(group_id);
        self
    }

    fn build(&mut self) -> ApplicationParameter {
        ApplicationParameter {
            version: self.version.take().expect("Version is required"),
            plugin_name: self.plugin_name.take().expect("Plugin name is required"),
            group_id: self.group_id.take().expect("Group ID is required"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_build_application_parameter() {
        // GIVEN the application parameter builder
        let mut builder = DefaultApplicationParameterBuilder {
            version: None,
            plugin_name: None,
            group_id: None,
        };

        // WHEN we set the version, plugin name and group id
        let application_parameter = builder
            .set_version("1.21".to_string())
            .set_plugin_name("plugin".to_string())
            .set_group_id("group".to_string())
            .build();

        // THEN the application parameter should be built
        assert_eq!("1.21", application_parameter.version);
        assert_eq!("plugin", application_parameter.plugin_name);
        assert_eq!("group", application_parameter.group_id);
    }

    #[test]
    #[should_panic(expected = "Version is required")]
    fn should_not_build_application_parameter_without_version() {
        // GIVEN the application parameter builder
        let mut builder = DefaultApplicationParameterBuilder {
            version: None,
            plugin_name: None,
            group_id: None,
        };

        // WHEN we build the application parameter without setting the version
        // THEN it should panic
        builder.build();
    }

    #[test]
    #[should_panic(expected = "Plugin name is required")]
    fn should_not_build_application_parameter_without_plugin_name() {
        // GIVEN the application parameter builder
        let mut builder = DefaultApplicationParameterBuilder {
            version: Some("1.21".to_string()),
            plugin_name: None,
            group_id: None,
        };

        // WHEN we build the application parameter without setting the plugin name
        // THEN it should panic
        builder.build();
    }

    #[test]
    #[should_panic(expected = "Group ID is required")]
    fn should_not_build_application_parameter_without_group_id() {
        // GIVEN the application parameter builder
        let mut builder = DefaultApplicationParameterBuilder {
            version: Some("1.21".to_string()),
            plugin_name: Some("plugin".to_string()),
            group_id: None,
        };

        // WHEN we build the application parameter without setting the group id
        // THEN it should panic
        builder.build();
    }
}
