use std::error;

use chrono::SecondsFormat;
use cloudevents::{AttributesReader, Event};
use serde_json;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub(crate) fn pretty_print(ce: &str) -> Result<String> {
    return serde_json::from_str(ce)
        .map_err(|e| e.into())
        .and_then(|v| present(v));
}

pub fn present(ce: Event) -> Result<String> {
    let mut result = String::new();
    result.push_str("☁️  cloudevents.Event\n");
    result.push_str("Validation: valid\n");
    result.push_str("Context Attributes,\n");
    result.push_str(&format!("  specversion: {}\n", ce.specversion()));
    result.push_str(&format!("  type: {}\n", ce.ty()));
    result.push_str(&format!("  source: {}\n", ce.source()));
    result.push_str(&format!("  id: {}\n", ce.id()));
    if let Some(time) = ce.time() {
        let t = time.to_rfc3339_opts(SecondsFormat::AutoSi, true);
        result.push_str(&format!("  time: {}\n", t));
    }
    if let Some(subject) = ce.subject() {
        result.push_str(&format!("  subject: {}\n", subject));
    }
    if let Some(dataschema) = ce.dataschema() {
        result.push_str(&format!("  dataschema: {}\n", dataschema));
    }
    if let Some(datacontenttype) = ce.datacontenttype() {
        result.push_str(&format!("  datacontenttype: {}\n", datacontenttype));
    }

    let mut any_exts = false;
    for (k, v) in ce.iter_extensions() {
        if !any_exts {
            result.push_str("Extensions,\n");
            any_exts = true;
        }
        result.push_str(&format!("  {}: {}\n", k, v));
    }
    if let Some(data) = ce.data() {
        let d = match data {
            cloudevents::Data::Json(v) => {
                let r = serde_json::to_string_pretty(v);
                r.map(|s| indent::indent_all_by(2, s))?
            }
            cloudevents::Data::Binary(v) => String::from_utf8(v.to_vec())?,
            cloudevents::Data::String(s) => s.to_string(),
        };

        result.push_str("Data,\n");
        result.push_str(&format!("{}\n", d));
    }

    return Ok(result);
}

#[cfg(test)]
mod tests {
    use crate::pp::pretty_print;

    #[test]
    fn test_pretty_print() {
        let json = include_str!("test/example-ce.json");
        let want = include_str!("test/example-ce.txt");
        let got = pretty_print(json);
        assert!(got.is_ok());
        assert_eq!(got.unwrap(), want);
    }

    #[test]
    fn test_not_json() {
        let json = "not json";
        let got = pretty_print(json);
        assert!(got.is_err());
    }

    #[test]
    fn test_not_ce() {
        let json = "{}";
        let got = pretty_print(json);
        assert!(got.is_err());
    }

    #[test]
    fn test_invalid_ce() {
        let json = r#"{"specversion": "1.0"}"#;
        let got = pretty_print(json);
        assert!(got.is_err());
    }
}
