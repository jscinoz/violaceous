extern crate libpurple_sys as purple;

use std::os::raw::c_int;
use self::purple::{guint, gpointer, GSourceFunc};

use util::Binding;

// XXX: Do we have function members here, or provide them on an impl?
pub struct UiOps {
    raw: *mut purple::PurpleEventLoopUiOps,
}

impl Binding for UiOps {
    type Raw = *mut purple::PurpleEventLoopUiOps;

    fn from_raw(raw: Self::Raw) -> Self {
        UiOps { raw: raw }
    }

    fn to_raw(self) -> Self::Raw {
        self.raw
    }
}

impl UiOps {
    extern "C"
    fn timeout_add(interval: guint, func: GSourceFunc, data: gpointer) -> guint {
        println!("interval from purple: {}", interval);
        // TODO
        1
    }
/*
    fn timeout_remove(handle: guint) -> gboolean {
        // TODO
    }

    fn input_add(fd: c_int, cond: purple::PurpleInputCondition,
                 func: purple::PurpleInputFunction, user_data: gpointer) {
        // TODO
    }

    fn input_remove(hanle: guint) -> gboolean {
        // TODO
    }
    */

}

// XXX: Should we implement Drop for UiOps to clear the internal struct?

impl Default for UiOps {
    fn default() -> Self {
        UiOps {
            raw: Box::into_raw(Box::new(purple::PurpleEventLoopUiOps {
                timeout_add: Some(Self::timeout_add),
                timeout_remove: None,
                input_add: None,
                input_remove: None, 
                input_get_error: None, 
                timeout_add_seconds: None, 
                /*
                timeout_remove: Self::timeout_remove,
                input_add: Self::input_add,
                input_remove: Self::input_remove,
                input_get_error: Self::input_get_error,
                timeout_add_seconds: Self::timeout_add_seconds,
                */
                _purple_reserved2: None,
                _purple_reserved3: None,
                _purple_reserved4: None,
            }))
        }
    }
}

pub fn set_ui_ops(ui_ops: UiOps) {
    unsafe {
        purple::purple_eventloop_set_ui_ops(ui_ops.to_raw());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_ui_ops_works() {
        // Shouldn't segfault
        set_ui_ops(UiOps::default());
    }
}
