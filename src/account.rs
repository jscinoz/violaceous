extern crate libpurple_sys as purple;

use std::ffi::CString;

pub struct PurpleAccount {
    username: String,
//    alias: String,
//    password: String,
//    user_info: String,
//    buddy_icon_path: String,
//    remember_pass: bool,
    protocol_id: String,
//    gc: PurpleConnection,
//    disconnecting: bool,
    // TODO: Other fields from native struct
}

impl From<*mut purple::PurpleAccount> for PurpleAccount {
    fn from(account: *mut purple::PurpleAccount) -> Self {
        unsafe {
            let account = *account;

            PurpleAccount {
                username: CString::from_raw(account.username).into_string().unwrap(),
                protocol_id: CString::from_raw(account.protocol_id).into_string().unwrap(),
            }
        }
    }
}

/*
impl Into<*mut purple::PurpleAccount> for PurpleAccount {
    fn into(self) -> *mut purple::PurpleAccount {

    }
}
*/

impl PurpleAccount {
    fn new<S: Into<Vec<u8>>>(username: S, protocol_id: S) -> Self {
        let username = CString::new(username).unwrap();
        let protocol_id = CString::new(protocol_id).unwrap();

        unsafe {
            let account = purple::purple_account_new(
                username.into_raw(), protocol_id.into_raw());

            Self::from(account)
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_account() {
        let expected_username = "jscinoz@jscinoz.so";
        let expected_protocol_id = "jabber";
        let PurpleAccount { username, protocol_id, .. } =
            PurpleAccount::new(expected_username, expected_protocol_id);

        assert_eq!(expected_username, username);
        assert_eq!(expected_protocol_id, protocol_id);
    }
}
*/
