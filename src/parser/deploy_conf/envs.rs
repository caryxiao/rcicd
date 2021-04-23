use std::collections::HashMap;

#[derive(Debug)]
pub struct Vars<'a> {
    map: HashMap<&'a str, String>
}

/// 项目 env 环境变量配置
#[derive(Debug)]
pub struct Env<'a> {
    name: &'a str,
    vars: Box<Vars<'a>>
}

/// 每个环境所有的变量配置, 使用的hashmap方式存储
impl<'a> Vars<'a> {
    pub fn new() -> Self {
        Vars {
            map: HashMap::new()
        }
    }

    pub fn insert(&'a mut self, k: &'a str, v: String) -> Option<String> {
        self.map.insert(k, v)
    }

    pub fn get(&'a self, k: &'a str) -> Option<&'a String> {
        self.map.get(k)
    }
}

impl<'a> Env<'a> {
    pub fn new(name: &'a str) -> Self {
        Env {
            name,
            vars: Box::new(Vars::new())
        }
    }

    pub fn get_name(&'a self) -> &'a str {
        self.name
    }

    pub fn set_name(&'a mut self, name: &'a str) {
        self.name = name;
    }
}