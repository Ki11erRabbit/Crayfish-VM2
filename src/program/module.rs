use std::collections::HashMap;
use crate::program::FunctionPath;
use crate::value::function::Function;

pub struct Module {
    module_name: Box<str>,
    functions: HashMap<Box<str>, Function>,
    string_table: Vec<Box<str>>,
    sub_modules: HashMap<Box<str>, Module>,
}


impl Module {

    pub fn new(module_name: &str, functions: HashMap<Box<str>, Function>, string_table: Vec<Box<str>>, sub_modules: HashMap<Box<str>, Module>) -> Self {
        Module {
            module_name: module_name.to_string().into(),
            functions,
            string_table,
            sub_modules,
        }
    }

    pub fn get_function(&self, path: &FunctionPath) -> Option<&Function> {
        let mut module = self;
        for part in path.path.iter().take(path.path.len() - 1) {
            module = module.sub_modules.get(part)?;
        }

        match module.functions.get(path.path.last()?) {
            Some(function) => Some(function),
            None => None
        }
    }

    pub fn add_function(&mut self, path: &str, function: Function) {
        self.functions.insert(path.to_string().into(), function);
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
            module_name: "".to_string().into(),
            functions: HashMap::new(),
            string_table: Vec::new(),
            sub_modules: HashMap::new(),
        }
    }
}