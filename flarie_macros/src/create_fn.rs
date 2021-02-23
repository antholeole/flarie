use proc_macro2::{TokenStream, Literal};
use proc_quote::quote;
use quote::format_ident;
use syn::{spanned::Spanned, AttributeArgs, ItemFn, Lit, LitStr, NestedMeta};
use crate::path_params::{find_path_type, generate_match_n_path_params_fn, generate_match_no_path_params, generate_param_data_extractor};


pub fn parse_route_fn(route_fn: ItemFn, meta_data: AttributeArgs, route_type: TokenStream) -> syn::Result<TokenStream> {
    let fn_name = route_fn.sig.ident;
    let (path_param_names, path_param_types, is_bundled) = find_path_type(&route_fn.sig.inputs);
    let block = route_fn.block;
    let local_private_fn = format_ident!("_{}", fn_name);

    let path_to_match = meta_data
        .into_iter()
        .enumerate()
        .map(|(index, nested_meta)| match index {
            0 => match nested_meta {
                NestedMeta::Lit(some_lit) => match some_lit {
                    Lit::Str(lit_str) => Ok(lit_str),
                    _ => Err(syn::Error::new(some_lit.span(), "Expected string literal")),
                },
                other_meta => Err(syn::Error::new(
                    other_meta.span(),
                    "Expected string literal",
                )),
            },
            _ => Err(syn::Error::new(nested_meta.span(), "Too many args")),
        })
        .collect::<syn::Result<Vec<LitStr>>>()?
        .remove(0);

    let path_params = quote!((#(#path_param_types),*));
    let path_params_extractor = generate_param_data_extractor(path_param_names, is_bundled);
    let path_param_fn = match &path_param_types.len() {
        0 => generate_match_no_path_params(),
        _ => generate_match_n_path_params_fn(path_param_types)
    };

    Ok(quote! {
        pub fn #fn_name() -> flarie::Route<#path_params> {
            fn #local_private_fn(__data: flarie::RouteData<#path_params>) -> flarie::Response {
                #path_params_extractor
                #block
            }

            #path_param_fn

            flarie::Route::new(#local_private_fn, _match_path_params, #path_to_match)
        }
    })
}

