use crate::parser::deploy_conf::envs::Env;
use serde_yaml::Value as serdeValue;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

mod envs;

#[derive(Debug)]
pub struct Conf {
    envs: HashMap<String, envs::Env>,
}

trait ConfOperation {
    fn set_env(&mut self, env: envs::Env);
    fn set_envs(&mut self, envs: HashMap<String, envs::Env>);
}

impl Conf {
    pub fn new() -> Self {
        Conf {
            envs: HashMap::new(),
        }
    }

    pub fn from_yaml_file(file: &str) -> Result<Conf, Box<dyn Error>> {
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
        if let serdeValue::Mapping(serde_mapping) = content {
            let config_envs = serde_mapping
                .get(&serdeValue::String("envs".to_string()))
                .unwrap();
            conf.set_envs(init_envs(config_envs.as_sequence()));
        }

        Ok(conf)
    }
}

impl ConfOperation for Conf {
    fn set_env(&mut self, env: Env) {
        self.envs.insert(env.get_name().to_string(), env);
    }

    fn set_envs(&mut self, envs: HashMap<String, Env>) {
        self.envs = envs;
    }
}

fn init_envs(origin_envs: Option<&serde_yaml::Sequence>) -> HashMap<String, Env> {
    let envs_op = envs::yaml_convert_envs(origin_envs);
    envs_op.unwrap()
}

fn make_serde_str<'a>(s: &str) -> serdeValue {
    serde_yaml::from_str(s).unwrap()
}
