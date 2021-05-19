use crate::items;
use crate::items::jobs::Job;
use serde_yaml::Value as serdeValue;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::{env, fs};

#[derive(Debug)]
pub struct Conf {
    envs: HashMap<String, items::envs::Env>,
    jobs: HashMap<String, items::jobs::Job>,
}

trait ConfOperation {
    fn set_env(&mut self, env: items::envs::Env);
    fn set_envs(&mut self, envs: HashMap<String, items::envs::Env>);
    fn set_jobs(&mut self, jobs: HashMap<String, items::jobs::Job>);
}

impl Conf {
    pub fn new() -> Self {
        Conf {
            envs: Default::default(),
            jobs: Default::default(),
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
            let yaml_raw_envs = serde_mapping
                .get(&serdeValue::String("envs".to_string()))
                .unwrap();
            conf.set_envs(items::envs::from_yaml(yaml_raw_envs.as_sequence()).unwrap());
            let yaml_raw_deploy = serde_mapping.get(&items::make_serde_str("jobs")).unwrap();
            let jobs = items::jobs::from_yaml(yaml_raw_deploy.as_sequence());
            conf.set_jobs(jobs);
        }

        Ok(conf)
    }
}

impl ConfOperation for Conf {
    fn set_env(&mut self, env: items::envs::Env) {
        self.envs.insert(env.get_name().to_string(), env);
    }

    fn set_envs(&mut self, envs: HashMap<String, items::envs::Env>) {
        self.envs = envs;
    }

    fn set_jobs(&mut self, jobs: HashMap<String, Job>) {
        self.jobs = jobs;
    }
}
