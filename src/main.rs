use std::{
    env,
    io::{self, BufReader},
};

use minecraft_dev_tool::domain::{parser, plugin_template::{self, PluginTemplateBuilder}};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut buffer = BufReader::new(io::stdin());

    let plugin_version: String = parser::get_template_parameter(
        args.as_slice(),
        1,
        &mut buffer,
        "Enter the plugin version:",
    );
    let plugin_name: String =
        parser::get_template_parameter(args.as_slice(), 2, &mut buffer, "Enter the plugin name:");
    let plugin_group_id: String = parser::get_template_parameter(
        args.as_slice(),
        3,
        &mut buffer,
        "Enter the group id of the project:",
    );

    let mut builder = plugin_template::DefaultPluginTemplateBuilder::default();

    let application_parameter = builder
        .set_version(plugin_version)
        .set_plugin_name(plugin_name)
        .set_group_id(plugin_group_id)
        .build();

    println!("{:?}", application_parameter);
}
