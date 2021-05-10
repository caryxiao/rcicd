use crate::parser::config::Conf;
use clap::{load_yaml, App};

mod deploy;
mod parser;
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    // 分析需要执行build还是publish
    // build 需要分析 rcicd.yaml 配置文件, 组合配置文件中的环境变量, 替换和覆盖值
    // build 步骤需要 按照指定env 和 app生成指定的配置文件,
    if let Some((sub_cmd, sub_cmd_args)) = matches.subcommand() {
        // dbg!(sub_cmd, sub_cmd_args);
        if sub_cmd.eq("build") {
            if let Some(conf_file) = sub_cmd_args.value_of("config") {
                dbg!(conf_file);
                let prj_conf = parser::config::Conf::from_yaml_file(conf_file).expect("err");
                dbg!(prj_conf);
                deploy::runner::run();
            }
        }
    }
}
