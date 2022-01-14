use proc_macro::TokenStream;

#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    init_impl(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn init_impl(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let input: syn::Ident = syn::parse2(input)?;
    Ok(quote::quote! {
        #[no_mangle]
        pub unsafe extern "C" fn napi_register_module_v1(
            env: node_api_rs::api::napi_env,
            exports: node_api_rs::api::napi_value
        ) -> node_api_rs::api::napi_value {
            let _ = std::panic::catch_unwind(|| #input(env, exports));
            exports
        }
    })
}
