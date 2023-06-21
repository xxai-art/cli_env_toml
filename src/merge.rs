use toml::Value;

pub fn merge(merged: &mut Value, value: &Value) {
  match value {
    toml::Value::String(_)
    | toml::Value::Integer(_)
    | toml::Value::Float(_)
    | toml::Value::Boolean(_)
    | toml::Value::Datetime(_) => *merged = value.clone(),
    toml::Value::Array(x) => match merged {
      toml::Value::Array(merged) => {
        for (k, v) in x.iter().enumerate() {
          match merged.get_mut(k) {
            Some(x) => merge(x, v),
            None => {
              let _ = merged.insert(k.clone(), v.clone());
            }
          }
        }
      }
      _ => *merged = value.clone(),
    },
    toml::Value::Table(x) => match merged {
      toml::Value::Table(merged) => {
        for (k, v) in x.iter() {
          match merged.get_mut(k) {
            Some(x) => merge(x, v),
            None => {
              let _ = merged.insert(k.clone(), v.clone());
            }
          }
        }
      }
      _ => *merged = value.clone(),
    },
  }
}

#[test]
fn test() {
  let a = "[a]
b=1
c=\"x\"
f=3
";
  let b = "[a]
d=3
e=2
f=true";

  let mut a = a.parse().unwrap();
  let b = b.parse().unwrap();
  merge(&mut a, &b);
  let a = toml::ser::to_string_pretty(&a).unwrap();
  assert_eq!(
    a,
    "[a]
b = 1
c = \"x\"
d = 3
e = 2
f = true
"
  )
}
