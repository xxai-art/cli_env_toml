[‼️]: ✏️README.mdt

# toml_conf

用于从环境变量、命令行参数生成 toml，然后与已有的 toml 文件做配置合并。

先设置环境变量 [`source ./env`](./env):

```rust
export TEST_server__host=127.0.0.1
export TEST_site__title="xxAI.Art - 我们计算艺术"
export TEST_grpc_port=9999
export TEST_site__xxai_art__mail=xxai.art@gmail.com
export TEST_compress=true
```

用法见 [./src/lib.rs](./src/lib.rs) :

```rust
use std::env::vars;

mod env;
pub use env::{env_with_prefix, kv_toml};
use serde::de::DeserializeOwned;
mod merge;
use std::path::Path;

pub use merge::merge;

#[test]
fn test() {
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
pub fn cli_env_toml_value(
  cli: Option<Vec<impl AsRef<str>>>,
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
    let env_toml = kv_toml(env_with_prefix(vars(), env_prefix), "__");
    merge(&mut config, &env_toml.parse().unwrap());
  }

  // 从命令行参数读取配置
  if let Some(cli) = cli {
    let cli: Vec<(String, String)> = cli
      .iter()
      .filter_map(|s| {
        let s = s.as_ref();
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

pub fn cli_env_toml_str(
  cli: Option<Vec<impl AsRef<str>>>,
  env_prefix: impl AsRef<str>,
  toml_path: Option<impl AsRef<Path>>,
) -> anyhow::Result<String> {
  let config = cli_env_toml_value(cli, env_prefix, toml_path)?;
  Ok(toml::ser::to_string_pretty(&config)?)
}

pub fn cli_env_toml<T: DeserializeOwned>(
  cli: Option<Vec<impl AsRef<str>>>,
  env_prefix: impl AsRef<str>,
  toml_path: Option<impl AsRef<Path>>,
) -> anyhow::Result<T> {
  let config = cli_env_toml_value(cli, env_prefix, toml_path)?;
  Ok(T::deserialize(config)?)
}
```

输出为 :

## toml config

```toml
grpc_port=1234
mysql_port=1235

[site]
title="a b c"
password="xyz"

[site.xxai_art]
hide=true
    

## convert env into toml

```toml
compress=true
grpc_port=9999
[site]
title="xxAI.Art - 我们计算艺术"
[site.xxai_art]
mail="xxai.art@gmail.com"
[server]
host="127.0.0.1"

```
## merge config and env

```toml
compress = true
grpc_port = 9999
mysql_port = 1235

[server]
host = "127.0.0.1"

[site]
password = "xyz"
title = "xxAI.Art - 我们计算艺术"

[site.xxai_art]
hide = true
mail = "xxai.art@gmail.com"

```
