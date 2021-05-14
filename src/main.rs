#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{load_yaml, App};
use rcicd_config::config::Conf;
use rcicd_deploy;

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )]);
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
                let prj_conf = Conf::from_yaml_file(conf_file).expect("err");
                // dbg!(prj_conf);
                rcicd_deploy::runner::run();
            }
        }
    }
}
