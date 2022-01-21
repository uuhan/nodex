use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::{api, prelude::*};

impl NapiExtendedErrorInfo {
    pub fn error_message(&self) -> &str {
        unsafe { CStr::from_ptr(self.error_message).to_str().unwrap() }
    }
}
