use semver::Version;
use toml_edit::DocumentMut;

pub trait IntoTomlValue {
    fn into_toml_value(self) -> toml_edit::Value;
}

pub trait ToInlineTable {
    fn to_inline_table(&self) -> toml_edit::InlineTable;
}

impl IntoTomlValue for String {
    fn into_toml_value(self) -> toml_edit::Value {
        self.into()
    }
}

impl IntoTomlValue for &str {
    fn into_toml_value(self) -> toml_edit::Value {
        self.into()
    }
}

impl IntoTomlValue for toml_edit::Array {
    fn into_toml_value(self) -> toml_edit::Value {
        self.into()
    }
}

impl IntoTomlValue for toml_edit::InlineTable {
    fn into_toml_value(self) -> toml_edit::Value {
        self.into()
    }
}

impl IntoTomlValue for &Version {
    fn into_toml_value(self) -> toml_edit::Value {
        self.to_string().into()
    }
}

impl IntoTomlValue for Version {
    fn into_toml_value(self) -> toml_edit::Value {
        self.to_string().into()
    }
}

impl IntoTomlValue for Vec<Version> {
    fn into_toml_value(self) -> toml_edit::Value {
        toml_edit::Value::Array(toml_edit::Array::from_iter(
            self.into_iter().map(|v| v.to_string()),
        ))
    }
}

impl IntoTomlValue for &Vec<Version> {
    fn into_toml_value(self) -> toml_edit::Value {
        toml_edit::Value::Array(toml_edit::Array::from_iter(
            self.iter().map(|v| v.to_string()),
        ))
    }
}

impl IntoTomlValue for Vec<String> {
    fn into_toml_value(self) -> toml_edit::Value {
        toml_edit::Value::Array(toml_edit::Array::from_iter(self))
    }
}

impl IntoTomlValue for &Vec<String> {
    fn into_toml_value(self) -> toml_edit::Value {
        toml_edit::Value::Array(toml_edit::Array::from_iter(self.iter().map(String::as_str)))
    }
}

pub fn set_optional<V: IntoTomlValue>(doc: &mut DocumentMut, key: &str, val: Option<V>) {
    match val {
        Some(v) => doc[key] = toml_edit::value(v.into_toml_value()),
        None => {
            doc.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_optional_some() {
        let mut doc = DocumentMut::new();

        set_optional(&mut doc, "key1", Some("value1"));
        assert_eq!(doc["key1"].as_str(), Some("value1"));
    }

    #[test]
    fn test_set_optional_none() {
        let mut doc = DocumentMut::new();
        doc["key1"] = toml_edit::value("value1");

        set_optional(&mut doc, "key1", None::<&str>);
        assert!(doc.get("key1").is_none());
    }

    #[test]
    fn test_set_optional_vec() {
        let mut doc = DocumentMut::new();

        set_optional(
            &mut doc,
            "key1",
            Some(vec!["a".to_string(), "b".to_string()]),
        );
        let arr = doc["key1"].as_array().unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0).unwrap().as_str(), Some("a"));
        assert_eq!(arr.get(1).unwrap().as_str(), Some("b"));
    }

    #[test]
    fn test_set_optional_vec_none() {
        let mut doc = DocumentMut::new();
        doc["key1"] = toml_edit::value(toml_edit::Array::from_iter(["a", "b"]));

        set_optional(&mut doc, "key1", None::<Vec<String>>);
        assert!(doc.get("key1").is_none());
    }
}
