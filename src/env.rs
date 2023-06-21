use std::collections::HashMap;

pub fn env_with_prefix(
  iter: impl IntoIterator<Item = (String, String)>,
  prefix: impl AsRef<str>,
) -> HashMap<String, String> {
  let prefix = prefix.as_ref();
  let len = prefix.len();
  iter
    .into_iter()
    .filter(|(key, _)| key.starts_with(prefix))
    .map(|(k, v)| (k[len..].into(), v))
    .collect()
}

/// 格式文本生成 toml
pub fn kv_toml(iter: impl IntoIterator<Item = (String, String)>, split: impl AsRef<str>) -> String {
  let split = split.as_ref();
  let mut section = HashMap::<String, Vec<String>>::new();
  let mut root = HashMap::<String, String>::new();
  let mut r = String::new();
  for (k, v) in iter {
    let li = k.split(split).collect::<Vec<_>>();
    let len = li.len();
    if len == 1 {
      root.insert(k, v);
      // r += &format!("{k}={v}\n")
    } else if len > 1 {
      let len = len - 1;
      let k = li[len];
      let kv = format!("{k}={v}");
      let s = li[..len].join(".");
      let s = format!("[{s}]");
      if let Some(mut li) = section.get_mut(&s) {
        li.push(kv);
      } else {
        section.insert(s, vec![kv]);
      }
    }
  }
  for (k, v) in root {
    r += &k;
    r.push('=');
    r += &v;
    r.push('\n');
  }
  for (k, li) in section {
    r += &k;
    r.push('\n');
    for i in li {
      r += &i;
      r.push('\n');
    }
  }
  r
}
