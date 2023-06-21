mod env;
pub use env::{env_with_prefix, kv_toml};
mod merge;
use std::path::Path;

pub use merge::merge;

#[test]
fn test() {
  use std::env::vars;

  let config = "grpc_port=1234
mysql_port=1235

[site]
title=\"a b c\"
password=\"xyz\"

[site.xxai_art]
hide=true
    ";
  println!("\n## toml config\n\n```toml\n{config}\n");

  let env = env_with_prefix(vars(), "TEST_");

  let env_toml = kv_toml(env, "__");
  println!("## convert env into toml\n\n```toml\n{env_toml}\n```");

  let mut config = config.parse().unwrap();
  merge(&mut config, &env_toml.parse().unwrap());
  let config = toml::ser::to_string_pretty(&config).unwrap();
  println!("## merge config and env\n\n```toml\n{config}\n```");
}

/// 从 命令行、环境变量、配置文件 读取参数（前面的会覆盖后面的设置）
pub fn config_cli_env_toml(
  cli: Option<Vec<impl std::string::ToString>>,
  env_prefix: impl AsRef<str>,
  toml_path: Option<impl AsRef<Path>>,
) -> anyhow::Result<toml::Value> {
  let toml = if let Some(path) = toml_path {
    std::fs::read_to_string(path)?
  } else {
    String::new()
  };

  let mut config = toml.parse()?;

  // 从环境变量读取配置
  {
    let env_toml = kv_toml(std::env::vars(), env_prefix);
    merge(&mut config, &env_toml.parse().unwrap());
  }

  // 从命令行参数读取配置
  if let Some(cli) = cli {
    let cli: Vec<(String, String)> = cli
      .iter()
      .filter_map(|s| {
        let s = s.to_string();
        if let Some(index) = s.find('=') {
          let (left, right) = s.split_at(index);
          Some((left.to_string(), right[1..].to_string()))
        } else {
          None
        }
      })
      .collect();
    let cli_toml = kv_toml(cli, ".");
    merge(&mut config, &cli_toml.parse().unwrap());
  };
  Ok(config)
}
