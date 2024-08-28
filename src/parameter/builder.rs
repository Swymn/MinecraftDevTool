#[derive(Debug)]
#[allow(dead_code)]
pub struct ApplicationParameter<'a> {
    version: f32,
    plugin_name: &'a str,
    group_id: &'a str,
}

#[derive(Default)]
pub struct DefaultApplicationParameterBuilder<'a> {
    version: Option<f32>,
    plugin_name: Option<&'a str>,
    group_id: Option<&'a str>,
}

pub trait ApplicationParameterBuilder<'a> {
    type OutputType;

    fn set_version(&mut self, version: f32) -> &mut Self;
    fn set_plugin_name(&mut self, plugin_name: &'a str) -> &mut Self;
    fn set_group_id(&mut self, group_id: &'a str) -> &mut Self;

    fn build(&self) -> Self::OutputType;
}

impl<'a> ApplicationParameterBuilder<'a> for DefaultApplicationParameterBuilder<'a> {
    type OutputType = ApplicationParameter<'a>;

    fn set_version(&mut self, version: f32) -> &mut Self {
        self.version = Some(version);
        self
    }

    fn set_plugin_name(&mut self, plugin_name: &'a str) -> &mut Self {
        self.plugin_name = Some(plugin_name);
        self
    }

    fn set_group_id(&mut self, group_id: &'a str) -> &mut Self {
        self.group_id = Some(group_id);
        self
    }

    fn build(&self) -> ApplicationParameter<'a> {
        ApplicationParameter {
            version: self.version.expect("Version is required"),
            plugin_name: self.plugin_name.expect("Plugin name is required"),
            group_id: self.group_id.expect("Group ID is required"),
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
            .set_version(1.21)
            .set_plugin_name("plugin")
            .set_group_id("group")
            .build();

        // THEN the application parameter should be built
        assert_eq!(1.21, application_parameter.version);
        assert_eq!("plugin", application_parameter.plugin_name);
        assert_eq!("group", application_parameter.group_id);
    }

    #[test]
    #[should_panic(expected = "Version is required")]
    fn should_not_build_application_parameter_without_version() {
        // GIVEN the application parameter builder
        let builder = DefaultApplicationParameterBuilder {
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
        let builder = DefaultApplicationParameterBuilder {
            version: Some(1.21),
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
        let builder = DefaultApplicationParameterBuilder {
            version: Some(1.21),
            plugin_name: Some("plugin"),
            group_id: None,
        };

        // WHEN we build the application parameter without setting the group id
        // THEN it should panic
        builder.build();
    }
}
