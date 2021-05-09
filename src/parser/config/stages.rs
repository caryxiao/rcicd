/// 步骤
pub struct Step<T> {
    /// 调用名称
    /// 比如sh, 目前只有sh
    invoke_name: String,
    value: T,
}

/// 阶段
pub struct Stage<T> {
    name: String,
    steps: Vec<Step<T>>,
}

pub fn deploy_from_yaml(origin_envs: Option<&serde_yaml::Sequence>) {}
