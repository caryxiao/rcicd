use crate::items::make_serde_str;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StepValue {
    String(String),
}

/// 步骤
#[derive(Debug, Clone)]
pub struct Step {
    /// 调用名称
    /// 比如sh, 目前只有sh
    action: String,
    value: StepValue,
}

/// 阶段
#[derive(Debug, Clone)]
pub struct Stage {
    name: String,
    steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Job {
    env_name: String,
    stages: Vec<Stage>,
}

impl Step {
    pub fn new(action: String, value: StepValue) -> Self {
        Step { action, value }
    }
}

impl Stage {
    pub fn new(name: String, steps: Vec<Step>) -> Self {
        Stage { name, steps }
    }
}

impl Job {
    pub fn new(env_name: String, stages: Vec<Stage>) -> Self {
        Job { env_name, stages }
    }

    pub fn get_stages(&self) -> Option<&Vec<Stage>> {
        Some(&self.stages)
    }
}

pub fn from_yaml(raw_yaml: Option<&serde_yaml::Sequence>) -> HashMap<String, Job> {
    let mut jobs: HashMap<String, Job> = HashMap::new();

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

            let mut stages: Vec<Stage> = Vec::new();

            for job_conf in job {
                let stage = create_stage(job_conf);
                stages.push(stage);
            }

            for env_name in env_names.iter() {
                jobs.insert(
                    env_name.to_string(),
                    Job::new(env_name.to_string(), stages.to_owned()),
                );
            }
        }
    }

    jobs
}

fn create_stage(raw_mapping: &serde_yaml::Mapping) -> Stage {
    let stage_name = raw_mapping
        .get(&make_serde_str("stage"))
        .and_then(|v| v.as_str())
        .unwrap();
    let raw_steps = match raw_mapping
        .get(&make_serde_str("steps"))
        .and_then(|v| v.as_sequence())
    {
        Some(r_steps) => r_steps.to_owned(),
        None => serde_yaml::Sequence::default(),
    };
    // debug!("stage name: {} {:#?}", stage_name, &raw_steps);

    let steps = create_steps(&raw_steps);
    let stage = Stage::new(stage_name.to_string(), steps);
    return stage;
}

fn create_steps(raw_seq: &serde_yaml::Sequence) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    for item in raw_seq {
        if let serde_yaml::Value::Mapping(step) = item {
            for (key, value) in step {
                let action = match key {
                    serde_yaml::Value::String(act) => act.to_string(),
                    _ => panic!("action 未解析成功, 请检查配置文件"),
                };
                let value = match value {
                    serde_yaml::Value::String(val) => val.to_string(),
                    _ => panic!("action 所对应的数据未解析成功, 请检查配置文件"),
                };

                let step = Step::new(action, StepValue::String(value));
                steps.push(step);
            }
        }
    }
    return steps;
}
