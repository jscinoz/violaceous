extern crate libpurple_sys as purple;

use std::path::Path;
use std::ffi::CString;

pub fn set_user_dir(dir: &Path) {
    let dir = dir.to_str().unwrap();

    unsafe {
        purple::purple_util_set_user_dir(CString::new(dir).unwrap().as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::Path;

    use super::*;

    #[test]
    fn can_set_user_dir() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let tmp_user_dir = Path::new(&out_dir).join("purple-user-dir");

        // Shouldn't segfault
        set_user_dir(&tmp_user_dir);
    }
}
