use std::ffi::{c_char, CStr, CString};
use std::slice::from_raw_parts_mut;

use crate::pp;

#[no_mangle]
pub extern "C" fn pp_print(ce_ptr: *mut u8) -> i32 {
    let ce = unsafe { CStr::from_ptr(ce_ptr as *const c_char) };
    let result = pp::pretty_print(ce.to_str().unwrap());
    if result.is_err() {
        eprintln!("Error: {}", result.expect_err("should be an error here"));
        return 1;
    }
    let str = result.unwrap();
    let len = str.len();
    let cstr = CString::new(str).unwrap();
    let bytes = cstr.as_bytes_with_nul();
    unsafe {
        let header_bytes = from_raw_parts_mut(ce_ptr, len + 1);
        header_bytes[..bytes.len()].copy_from_slice(bytes);
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr, CString};
    use std::slice::from_raw_parts_mut;

    use indoc::indoc;
    use serde_json::json;

    use crate::wasm::pp_print;

    #[test]
    fn create_destroy() {
        let res = execute_pp(String::from("{}"));
        assert!(res.is_none());
    }

    #[test]
    fn print() {
        let js = json!({
          "specversion": "1.0",
          "id": "60fd",
          "source": "test/0.1",
          "type": "dev.generic",
        });
        let ce = serde_json::to_string(&js).ok().unwrap();

        let res = execute_pp(ce);

        assert!(res.is_some());
        let got = res.unwrap();
        assert_eq!(
            got,
            indoc! {r###"
                ☁️  cloudevents.Event
                Validation: valid
                Context Attributes,
                  specversion: 1.0
                  type: dev.generic
                  source: test/0.1
                  id: 60fd
            "###}
        );
    }

    fn execute_pp(ce: String) -> Option<String> {
        let len = ce.len() * 10;
        let mut buf = vec![0u8; len];
        let cstr = CString::new(ce).unwrap();
        let bytes = cstr.as_bytes_with_nul();
        let ce_ptr = buf.as_mut_ptr();
        unsafe {
            let header_bytes = from_raw_parts_mut(ce_ptr, len);
            header_bytes[..bytes.len()].copy_from_slice(bytes);
        }

        let res = pp_print(ce_ptr);
        if res != 0 {
            return None;
        }
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) };
        let s = cstr.to_str().unwrap().to_string();
        return Some(s);
    }
}
