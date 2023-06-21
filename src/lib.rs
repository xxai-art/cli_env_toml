mod env;
pub use env::{env_with_prefix, kv_toml};
mod merge;

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

  let prefix = "TEST_";
  let env = env_with_prefix(vars(), prefix);

  let toml = kv_toml(env, "__");
  println!("## convert env into toml\n\n```toml\n{toml}\n```");

  let mut config = config.parse().unwrap();
  merge(&mut config, &toml.parse().unwrap());
  let config = toml::ser::to_string_pretty(&config).unwrap();
  println!("## merge config and env\n\n```toml\n{config}\n```");
}
