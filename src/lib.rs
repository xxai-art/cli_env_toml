mod env;
pub use env::{env_with_prefix, kv_toml};
mod merge;
pub use merge::merge;

#[test]
fn test() {
  let config = "[a]
b=1
c=\"x\"
f=3
";
  println!("\n--- toml config ---\n{config}");
  let _env = std::env::vars();
  let env = std::env::vars();
  let env = env_with_prefix(env, "TOML_CONF_");
  println!("--- env with prefix TOML_CONF_ ( set by direnv & ./.env ) ---\n");
  for (k, v) in &env {
    println!("{k}={v}");
  }

  let toml = kv_toml(env, "__");
  println!("\n--- convert env into toml ---\n{toml}");
  let mut config = config.parse().unwrap();

  merge(&mut config, &toml.parse().unwrap());
  let config = toml::ser::to_string_pretty(&config).unwrap();
  println!("--- merge config and env ---\n{config}");
}
