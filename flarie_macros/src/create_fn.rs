use quote::{quote, format_ident};
use syn::{AttributeArgs, ItemFn, spanned::Spanned, NestedMeta, Lit, LitStr};
use proc_macro2::TokenStream;

pub fn parse_route_fn(route_fn: ItemFn, meta_data: AttributeArgs) -> syn::Result<TokenStream> {
    let fn_name = route_fn.sig.ident;
    let block = route_fn.block;
    let local_private_fn = format_ident!("_{}", fn_name);


    //for now, we just grab the path and pop that from the vec - 
    //but this is open ended and we can return more
    let path_to_match = meta_data.into_iter().enumerate().map(|(index, nested_meta)| {
        match index {
            0 => match nested_meta {
                NestedMeta::Lit(some_lit) => match some_lit {
                    Lit::Str(lit_str) => Ok(lit_str),
                    _ => Err(syn::Error::new(some_lit.span(), "Expected string literal"))
                },
                other_meta => Err(syn::Error::new(other_meta.span(), "Expected string literal"))
            }
            _ => Err(syn::Error::new(nested_meta.span(), "Too many args"))
        }
    }).collect::<syn::Result<Vec::<LitStr>>>()?.remove(0);


    Ok(quote! {
        pub fn #fn_name() -> Route {
            fn #local_private_fn() -> Response 
                #block
            Route::new(#local_private_fn, #path_to_match)
        }
    })
}
