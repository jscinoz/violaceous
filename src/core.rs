extern crate libpurple_sys as purple;

use eventloop;

use ffiutil::Binding;
use std::ffi::CString;

pub struct UiOps {
    raw: *mut purple::PurpleCoreUiOps,
}

// TODO: Add methods to register callbacks on UiOps
/*
// OLD: probably not needed.
impl UiOps {
    fn ui_prefs_init(&self) {
        unsafe {
            let raw = *self.raw;

            // XXX: Should this be in the unsafe block?
            match raw.ui_prefs_init {
                Some(rawFn) => rawFn(),
                // XXX: Don't need this logging
                None => println!("No ui_prefs_init func")
            }
        }
    }
}
*/

impl Default for UiOps {
    fn default() -> Self {
        UiOps {
            raw: Box::into_raw(Box::new(purple::PurpleCoreUiOps {
                ui_prefs_init: None,
                debug_ui_init: None,
                ui_init: None,
                quit: None,
                get_ui_info: None,

                // These should not be exposed
                _purple_reserved1: None,
                _purple_reserved2: None,
                _purple_reserved3: None,
            }))
        }
    }
}

/*
impl Drop for UiOps {
    fn drop(&mut self) {
        println!("Dropping UiOps");
    }
}
*/

/*
impl From<*mut purple::PurpleCoreUiOps> for UiOps {
    fn from(raw: *mut purple::PurpleCoreUiOps) -> Self {
        UiOps { raw: raw }
    }
}

impl Into<*mut purple::PurpleCoreUiOps> for UiOps {
    fn into(self) -> *mut purple::PurpleCoreUiOps {
        self.raw
    }
}
*/

impl Binding for UiOps {
    type Raw = *mut purple::PurpleCoreUiOps;

    fn from_raw(raw: Self::Raw) -> Self {
        UiOps { raw: raw }
    }

    fn to_raw(self) -> Self::Raw {
        self.raw
    }
}

pub fn set_ui_ops(ui_ops: UiOps) {
    unsafe {
        purple::purple_core_set_ui_ops(ui_ops.to_raw());
    }
}

pub fn init<S: Into<Vec<u8>>>(ui_name: S) -> bool {
    let ui_name = CString::new(ui_name).unwrap();

    unsafe {
        let res = purple::purple_core_init(ui_name.as_ptr());

        println!("result is: {:?}", res);

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;

    use super::*;
    use eventloop;
    use util;

    #[test]
    fn can_set_ui_ops() {
        // Shouldn't segfault
        set_ui_ops(UiOps::default());
    }

    #[test]
    fn init_works() {
        // Need to set a temp user directory so libpurple doesn't use the user's
        // real direcctory (~/.purple)
        let out_dir = env::var("OUT_DIR").unwrap();
        let tmp_user_dir = Path::new(&out_dir).join("purple-user-dir");

        // Shouldn't segfault
        util::set_user_dir(&tmp_user_dir);

        // ui_ops must be set before init is called
        set_ui_ops(UiOps::default());
        // Same goes for eventloop_ui_ops
        eventloop::set_ui_ops(eventloop::UiOps::default());

        let result = init("Violaceous");

        assert!(result, "libpurple init failed");
    }
}
