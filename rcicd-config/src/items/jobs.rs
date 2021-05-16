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
    env_name: String,
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

            for job_conf in job {
                let stage = create_stage(job_conf);
            }
        }
    }
}

fn create_stage(raw_mapping: &serde_yaml::Mapping) {
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
    create_steps(&raw_steps);
}

fn create_steps(raw_seq: &serde_yaml::Sequence) {
    dbg!(raw_seq);
}
