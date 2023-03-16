use chrono::SecondsFormat;
use cloudevents::{AttributesReader, Event};
use serde_json;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pp(ce: &str) -> String {
    return serde_json::from_str(ce).map(|v| present(v)).unwrap();
}

fn present(ce: Event) -> String {
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
                r.map(|s| indent::indent_all_by(2, s)).unwrap()
            }
            cloudevents::Data::Binary(v) => String::from_utf8(v.to_vec()).unwrap(),
            cloudevents::Data::String(s) => s.to_string(),
        };

        result.push_str("Data,\n");
        result.push_str(&format!("{}\n", d));
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::pp;

    #[test]
    fn test_pp() {
        let json = include_str!("test/example-ce.json");
        let want = include_str!("test/example-ce.txt");
        let got = pp(json);
        assert_eq!(got, want);
    }
}
