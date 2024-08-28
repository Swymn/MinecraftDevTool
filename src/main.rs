use std::{
    env,
    io::{self, BufReader},
};

use minecraft_dev_tool::parameter::{
    builder, builder::ApplicationParameterBuilder, parameter_parser,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut buffer = BufReader::new(io::stdin());

    let plugin_version: f32 = parameter_parser::get_parameter(
        args.as_slice(),
        1,
        &mut buffer,
        "Enter the plugin version:",
    );
    let plugin_name: String =
        parameter_parser::get_parameter(args.as_slice(), 2, &mut buffer, "Enter the plugin name:");
    let plugin_group_id: String = parameter_parser::get_parameter(
        args.as_slice(),
        3,
        &mut buffer,
        "Enter the group id of the project:",
    );

    let mut builder = builder::DefaultApplicationParameterBuilder::default();

    let application_parameter = builder
        .set_version(plugin_version)
        .set_plugin_name(plugin_name.as_str())
        .set_group_id(plugin_group_id.as_str())
        .build();

    println!("{:?}", application_parameter);
}
