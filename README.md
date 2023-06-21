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
grpc_port=9999
compress=true
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
