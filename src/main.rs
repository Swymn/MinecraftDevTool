use std::{
    env,
    io::{self, BufReader},
};

use minecraft_dev_tool::domain::{parser, plugin_template::{self, PluginTemplateBuilder}};

const PLUGIN_VERSION_PROMPT: &str = "Enter the plugin version:";
const PLUGIN_NAME_PROMPT: &str = "Enter the plugin name:";
const GROUP_ID_PROMPT: &str = "Enter the group id of the project:";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut buffer = BufReader::new(io::stdin());

    let plugin_version: String = parser::get_template_parameter(
        args.as_slice(),
        1,
        &mut buffer,
        PLUGIN_VERSION_PROMPT,
    );
    let plugin_name: String =
        parser::get_template_parameter(args.as_slice(), 2, &mut buffer, PLUGIN_NAME_PROMPT);
    let plugin_group_id: String = parser::get_template_parameter(
        args.as_slice(),
        3,
        &mut buffer,
        GROUP_ID_PROMPT,
    );

    let mut builder = plugin_template::DefaultPluginTemplateBuilder::default();

    let application_parameter = builder
        .set_version(plugin_version)
        .set_plugin_name(plugin_name)
        .set_group_id(plugin_group_id)
        .build();

    println!("{:?}", application_parameter);
}
