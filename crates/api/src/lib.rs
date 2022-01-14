#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));
}

// useful macros
pub use node_api_macros::init;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_napi_true_() {
        assert_eq!(api::true_, 1);
    }
}
