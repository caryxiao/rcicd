use crate::config::make_serde_str;
use std::collections::HashMap;

pub const ENV_BASE_NAME: &'static str = "base";

#[derive(Debug, Clone)]
pub struct Vars {
    map: HashMap<String, String>,
}

/// 项目 env 环境变量配置
#[derive(Debug)]
pub struct Env {
    name: String,
    vars: Box<Vars>,
}

/// 每个环境所有的变量配置, 使用的hashmap方式存储
impl Vars {
    pub fn new() -> Self {
        Vars {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: &str, v: String) -> Option<String> {
        self.map.insert(k.to_string(), v)
    }

    pub fn get(&self, k: &str) -> Option<&String> {
        self.map.get(k)
    }
}

impl Env {
    pub fn new(name: &str) -> Self {
        Env {
            name: name.to_string(),
            vars: Box::new(Vars::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_vars(&mut self, vars: Box<Vars>) {
        self.vars = vars;
    }
}

fn get_base_envs(raw_yaml: Option<&serde_yaml::Sequence>) -> Option<Env> {
    let base = match raw_yaml {
        Some(seq) => seq.iter().find_map(|v| {
            let sub_map = v.as_mapping().unwrap();
            let key_name = &make_serde_str("env");
            if sub_map.contains_key(key_name) {
                if sub_map.get(key_name).unwrap().as_str() == Some(ENV_BASE_NAME) {
                    return Some(sub_map);
                }
            }
            None
        }),
        None => None,
    };

    match base {
        Some(b) => {
            let vars = b
                .get(&make_serde_str("vars"))
                .unwrap()
                .as_mapping()
                .unwrap();
            let mut env = Env::new("base");
            vars.iter().for_each(|(key, name)| {
                env.vars
                    .insert(key.as_str().unwrap(), name.as_str().unwrap().to_string());
            });
            Some(env)
        }
        None => None,
    }
}

pub fn envs_from_yaml(raw_yaml: Option<&serde_yaml::Sequence>) -> Option<HashMap<String, Env>> {
    let mut envs = HashMap::new();
    let base_env = get_base_envs(raw_yaml);
    if base_env.is_none() {
        return None;
    }

    let base_env = base_env.unwrap();

    for item in raw_yaml.unwrap() {
        let item_map = item.as_mapping().unwrap();
        let name = item_map.get(&make_serde_str("env"));
        if name.is_none() {
            continue;
        }

        let name = name.unwrap().as_str().unwrap();
        if name.is_empty() || name == ENV_BASE_NAME {
            continue;
        }

        let vars = item_map
            .get(&make_serde_str("vars"))
            .unwrap()
            .as_mapping()
            .unwrap();
        let mut cur_env = Env::new(name);
        cur_env.set_vars(base_env.vars.clone());

        vars.iter().for_each(|(key, value)| {
            let k = key.as_str().unwrap();
            let v = value.as_str().unwrap();
            cur_env.vars.insert(k, v.to_string());
        });
        envs.insert(name.to_string(), cur_env);
    }

    Some(envs)
}
