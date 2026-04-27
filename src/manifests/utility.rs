use toml_edit::DocumentMut;

pub fn set_optional<V: Into<toml_edit::Value>>(doc: &mut DocumentMut, key: &str, val: Option<V>) {
    match val {
        Some(v) => doc[key] = toml_edit::value(v),
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
}
