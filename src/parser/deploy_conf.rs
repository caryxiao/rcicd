use std::{env, fs};
use std::path::Path;
use serde_yaml::Value as serdeValue;
use std::error::Error;
use crate::parser::deploy_conf::envs::Env;
use std::collections::HashMap;

mod envs;

#[derive(Debug)]
pub struct Conf<'a> {
    envs: HashMap<String, envs::Env<'a>>,
}

trait ConfOperation<'a> {
    fn set_env(&mut self, env: envs::Env<'a>);
}

impl<'a> Conf<'a> {
    pub fn new() -> Self {
        Conf {
            envs: HashMap::new()
        }
    }

    pub fn from_yaml_file(file: &'a str) -> Result<Conf<'a>, Box<dyn Error>> {
        let mut final_file = Path::new(file).to_path_buf();
        // Determine whether the file is a relative path or an absolute path
        if file.starts_with("./") || !file.starts_with("/") {
            final_file = env::current_dir().unwrap().join(file);
        }

        if !final_file.is_file() {
            panic!("Project config file is not exist.")
        }

        let fd = fs::File::open(final_file.as_path())?;
        let content: serdeValue = serde_yaml::from_reader(fd)?;
        let mut conf = Self::new();
        let env1 = Env::new("aa");
        conf.set_env(env1);
        Ok(conf)
    }
}

impl<'a> ConfOperation<'a> for Conf<'a> {
    fn set_env(&mut self, env: Env<'a>) {
        self.envs.insert(env.get_name().to_string(), env);
    }
}