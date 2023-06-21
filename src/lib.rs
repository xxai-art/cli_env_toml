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
  let prefix = "TEST_";
  let env = env_with_prefix(env, prefix);
  println!("--- env with prefix {prefix} ( set by direnv & ./.env ) ---\n");
  for (k, v) in &env {
    let v = if v.starts_with('"') {
      format!("'{v}'")
    } else {
      v
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
