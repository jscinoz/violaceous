extern crate libpurple_sys as purple;

use util::Binding;
use std::ffi::CString;

pub struct UiOps {
    raw: *mut purple::PurpleCoreUiOps,
}

// Do we need this?
/*
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

impl Binding for UiOps {
    type NativeType = *mut purple::PurpleCoreUiOps;

    fn from_native(native: Self::NativeType) -> Self {
        UiOps { raw: native }
    }

    fn to_native(&self) -> Self::NativeType {
        self.raw
    }
}

pub fn set_ui_ops(ui_ops: UiOps) {
    unsafe {
        purple::purple_core_set_ui_ops(ui_ops.to_native());
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
/*
pub fn set_ui_ops(
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn set_ui_ops_works() {
        // Shouldn't panic or otherwise crash
        set_ui_ops(UiOps::default());
    }
    */

    #[test]
    fn init_works() {
        // ui_ops must be set before init is called
        set_ui_ops(UiOps::default());

        let result = init("Violaceous");

        assert!(result, "libpurple init failed");
    }
}
