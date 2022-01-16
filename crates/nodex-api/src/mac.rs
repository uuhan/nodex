// common macros
#[macro_export]
macro_rules! napi_module {
    ($init:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn napi_register_module_v1(
            env: nodex_api::api::napi_env,
            exports: nodex_api::api::napi_value,
        ) -> nodex_api::api::napi_value {
            let exports = nodex_api::value::JsObject::napi_module_exports(env, exports);
            let env = nodex_api::env::NapiEnv::from_raw(env);

            // TODO: deal with exception
            match std::panic::catch_unwind(|| $init(env, exports)) {
                Ok(r) => {}
                Err(e) => {}
            }

            exports.raw()
        }
    };
}

#[macro_export]
macro_rules! napi_call {
    // [out] result: napi function which has output
    (=$napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let status = $crate::api::$napi($($args),+, result.as_mut_ptr());
            if status.err() {
                return Err(status)
            }
            result.assume_init()
        }
    };

    ($napi:ident, $($args:expr),+ $(,)?) => {
        unsafe {
            let status = $crate::api::$napi($($args),+);
            if status.err() {
                return Err(status)
            }
        }
    }
}

#[macro_export]
macro_rules! napi_guard {
    ($version:expr) => {
        assert!(
            $version >= $crate::napi_version_guard(),
            "Oops, your node(napi {}) is too old to support napi >= {}",
            $version,
            $crate::napi_version_guard(),
        );
    }
}
