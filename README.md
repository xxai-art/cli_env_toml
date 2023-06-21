[‼️]: ✏️README.mdt

# toml_conf

用于从环境变量、命令行参数生成 toml，然后与已有的 toml 文件做配置合并。

用法见 [./src/lib.rs](./src/lib.rs)
```rust
mod env;
pub use env::{env_with_prefix, kv_toml};
mod merge;
pub use merge::merge;

#[test]
fn test() {
  use toml::Value;
  let config = "[a]
b=1
c=\"x\"
f=3
";
  println!("\n--- toml config ---\n{config}");
  let env = std::env::vars();
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
```

输出为

```
--- toml config ---
[a]
b=1
c="x"
f=3

--- env with prefix TOML_CONF_ ( set by direnv & ./.env ) ---

a__d="x"
c=13
a__b=0
a__e__f="x"

--- convert env into toml ---
c=13
[a.e]
f="x"
[a]
d="x"
b=0

--- merge config and env ---
c = 13

[a]
b = 0
c = "x"
d = "x"
f = 3

[a.e]
f = "x"
```
