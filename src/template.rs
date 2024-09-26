use std::{
    any::Any,
    io::{self, Error},
};

#[allow(dead_code)]
pub trait TemplateGenerator: Any {
    fn generate(&self) -> Result<String, Error>;
    fn as_any(&self) -> &dyn Any;
}

#[allow(dead_code)]
pub trait TemplateParameterReader {
    fn read_parameters<R: io::BufRead>(&mut self, args: Vec<String>, input_reader: &mut R);
}
