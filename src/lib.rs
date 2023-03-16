use std::ffi::{c_char, CStr, CString};
use std::mem::transmute;

mod alloc;
mod pp;

static mut DATA: *const CString = 0 as *const CString;

#[no_mangle]
pub unsafe extern "C" fn pp_free() {
    if DATA == 0 as *const CString {
        return;
    }
    unsafe {
        let _ = transmute::<*const CString, Box<CString>>(DATA);
        DATA = 0 as *const CString;
    }
}

#[no_mangle]
pub extern "C" fn pp_print(ce_ptr: *const c_char) -> *const c_char {
    let ce = unsafe {
        pp_free();
        CStr::from_ptr(ce_ptr)
    };
    let result = pp::pretty_print(ce.to_str().unwrap());
    if result.is_err() {
        return 0 as *const c_char;
    }
    let cstr = CString::new(result.unwrap()).unwrap();
    let boxed = Box::new(cstr);
    unsafe {
        DATA = transmute(boxed);
    }
    unsafe {
        return (&*DATA).as_ptr();
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr, CString};

    use indoc::indoc;
    use serde_json::json;

    use crate::{pp_free, pp_print};

    #[test]
    fn create_destroy() {
        let res = with_pp(String::from("{}"));
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

        let res = with_pp(ce);

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

    fn with_pp(ce: String) -> Option<String> {
        let cstr = CString::new(ce).unwrap();
        let ce_ptr = cstr.as_c_str();

        let res = pp_print(ce_ptr.as_ptr());
        if res == 0 as *const c_char {
            return None;
        }
        let cstr = unsafe { CStr::from_ptr(res) };
        let s = cstr.to_str().unwrap().to_string();
        unsafe { pp_free() };
        return Some(s);
    }
}
