use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    init_impl(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn init_impl(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input: syn::Ident = syn::parse2(input)?;
    Ok(quote! {
        #[no_mangle]
        pub unsafe extern "C" fn napi_register_module_v1(
            env: nodex_api::api::napi_env,
            exports: nodex_api::api::napi_value
        ) -> nodex_api::api::napi_value {
            let env = nodex_api::env::NapiEnv::from_raw(env);
            let exports = nodex_api::value::JsValue::from_raw(env, exports);

            // TODO: deal with exception
            match std::panic::catch_unwind(|| #input(env, exports)) {
                Ok(r) => {
                }
                Err(e) => {
                }
            }

            exports.raw()
        }
    })
}
