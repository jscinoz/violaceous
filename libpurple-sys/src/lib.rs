/*

use libc::{
    time_t, size_t, c_uchar
};

use glib_sys::{
    GType, GList, GHashTable,
    gconstpointer, gpointer, gboolean
};

// I have no idea what i'm doing here...

type gsize = size_t;
type guint = u32;
type guchar = c_uchar;
type gint64 = i64;
type guint64 = u64;
*/

extern crate va_list as va_list_crate;
extern crate libc;
extern crate glib_sys;
extern crate gobject_sys;

use va_list_crate::VaList as va_list;

use libc::{
    size_t,
    ssize_t as gssize,
    c_ulong as gulong,
    c_uchar as guchar,
    c_char as gchar,
    c_uint as guint,
    c_int as gint,
    c_long,
    time_t,
    tm as Struct_tm,
    FILE
};

use glib_sys::*;
/*{
    gboolean, gconstpointer, gpointer,
    GList, GHashTable, GSourceFunc, GByteArray, GSList, GCallback, GObject,
    GObjectClass
};
*/

use gobject_sys::*;
/*
use gobject_sys::{
    GCallback, GParameter,
    GObject as GObject_Real,
    GObjectClass as GObjectClass_Real,
};
*/

type gsize = size_t;
type gint64 = i64;
type guint64 = u64;
type guint8 = u8;
type guint16 = u16;
type guint32 = u32;
type __off_t = c_long;
type __off64_t = c_long;
type __ssize_t = c_long;

/*
impl Copy for GObject {}
impl Copy for GObjectClass {}
*/

/*
struct GObject(GObject_Real);
impl Copy for GObject {
    fn clone(&self) -> GObject {
        GObject(*self)
    }
}

impl <GObject_Real> for GObject {
    fn deref(&self) -> &GObject_Real {
        &self.value
    }
}

struct GObjectClass(GObjectClass_Real);

*/

// Suppress warnings in generated binding
#![allow(dead_code, non_camel_case_types, non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/purple.rs"));
