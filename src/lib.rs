extern crate glib_sys;

use glib_sys::*;

// Suppress warnings in generated binding
#[allow(dead_code, non_camel_case_types, non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/purple.rs"));
