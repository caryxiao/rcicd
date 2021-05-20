use rcicd_config::config::{Conf, GetJob};

pub fn run(proj_conf: &Conf, env: &str) {
    info!("Deploy Start:");
    if proj_conf.get_job(env).is_none() {
        panic!("没有找到目标环境的JOB");
    }
    let job = proj_conf.get_job(env);
    dbg!(&job);
}
