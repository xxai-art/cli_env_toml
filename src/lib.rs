mod env;
pub use env::{env_with_prefix, kv_toml};
mod merge;
pub use merge::merge;

#[test]
fn test() {
  let config = "grpc_port=1234
mysql_port=1235

[site]
title=\"a b c\"
password=\"xyz\"
";
  println!("\n--- toml config ---\n{config}");

  let prefix = "TEST_";
  println!("--- env with prefix {prefix} ( set by direnv & ./.env ) ---\n");
  let env = std::env::vars();
  let env = env_with_prefix(env, prefix);
  for (k, v) in &env {
    let v = if v.starts_with('"') {
      format!("'{v}'")
    } else {
      v.to_string()
    };
    println!("{prefix}{k}={v}");
  }

  let toml = kv_toml(env, "__");
  println!("\n--- convert env into toml ---\n{toml}");

  let mut config = config.parse().unwrap();
  merge(&mut config, &toml.parse().unwrap());
  let config = toml::ser::to_string_pretty(&config).unwrap();
  println!("--- merge config and env ---\n{config}");
}
