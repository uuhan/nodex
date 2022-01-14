use proc_macro::TokenStream;

#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    init_impl(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn init_impl(input: proc_macro2::TokenStream)
    -> syn::Result<proc_macro2::TokenStream>
{
    let input: syn::Ident = syn::parse2(input)?;
    Ok(quote::quote! {
        use node_api_rs::api;
        #[no_mangle]
        extern "C"
        pub unsafe fn napi_register_module_v1(
            env: api::napi_env,
            exports: api::napi_value
        ) -> api::napi_value {
            let env = api::Env::from_raw(env);
            let exports = api::Value::from_raw(env, exports);
            let result = std::panic::catch_unwind(|| #input(env, exports));
            let result = match result {
                Ok(result) => result,
                Err(panic_info) => {
                    env.throw_error("A panic occurred.");
                    return std::ptr::null_mut();
                },
            };
            let exports = match result {
                Ok(exports) => exports,
                Err(error) => {
                    if !env.is_exception_pending() {
                        env.throw_error(&format!("{}", error));
                    }
                    return std::ptr::null_mut();
                }
            };
            exports.raw()
        }
    })
}
