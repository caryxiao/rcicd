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

pub fn deploy_from_yaml(origin_envs: Option<&serde_yaml::Sequence>) {}
