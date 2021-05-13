pub mod envs;
pub mod jobs;

pub fn make_serde_str<'a>(s: &str) -> serde_yaml::Value {
    serde_yaml::from_str(s).unwrap()
}
