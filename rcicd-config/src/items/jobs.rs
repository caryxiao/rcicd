use crate::items::make_serde_str;

#[derive(Debug)]
pub enum StepValue {
    String(String),
}

/// 步骤
#[derive(Debug)]
pub struct Step {
    /// 调用名称
    /// 比如sh, 目前只有sh
    invoke_name: String,
    value: StepValue,
}

/// 阶段
#[derive(Debug)]
pub struct Stage {
    name: String,
    steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Job {
    name: String,
    stages: Vec<Stage>,
}

pub fn from_yaml(raw_yaml: Option<&serde_yaml::Sequence>) {
    for seq_item in raw_yaml.unwrap() {
        if let serde_yaml::Value::Mapping(raw_stage) = seq_item {
            let env_names: Vec<&str> = raw_stage
                .get(&make_serde_str("env"))
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .filter(|v| v.is_string())
                .map(|v| v.as_str().unwrap())
                .collect();
            let job: Vec<&serde_yaml::Mapping> = raw_stage
                .get(&make_serde_str("stages"))
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .filter(|v| v.is_mapping())
                .map(|v| v.as_mapping().unwrap())
                .collect();
            dbg!(env_names, job);
        }
    }
}
