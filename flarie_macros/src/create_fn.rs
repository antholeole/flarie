use proc_macro2::{TokenStream, Literal};
use proc_quote::quote;
use quote::format_ident;
use syn::{spanned::Spanned, AttributeArgs, ItemFn, Lit, LitStr, NestedMeta};

pub fn parse_route_fn(route_fn: ItemFn, meta_data: AttributeArgs) -> syn::Result<TokenStream> {
    let fn_name = route_fn.sig.ident;
    let block = route_fn.block;
    let local_private_fn = format_ident!("_{}", fn_name);

    //for now, we just grab the path and pop that from the vec -
    //but this is open ended and we can return more
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

    //TODO get this type and validate
    let path_params = syn::parse_str::<TokenStream>("(i32, String)").unwrap();
    let path_param_fn = generate_match_n_path_params_fn(&path_params, 2);
    println!("{}", path_param_fn);
    let num = 2;

    Ok(quote! {
        pub fn #fn_name() -> Route<#path_params> {
            fn #local_private_fn(data: RouteData<#path_params>) -> Response
                #block

            #path_param_fn

            Route::new(#local_private_fn, _match_path_params, #path_to_match)
        }
    })
}

fn generate_match_n_path_params_fn(
    path_params: &TokenStream, num: u32
) -> TokenStream {
    let vec_len_check = (0u32..num).map(|v| format_ident!("param_str_{}", v)).collect::<Vec::<syn::Ident>>();
    let string_parse = (0u32..num).map(Literal::u32_unsuffixed);
    let param_type_check = (0u32..num).map(|v| format_ident!("param_type_{}", v)).collect::<Vec::<syn::Ident>>();

    quote!(
        pub fn _match_path_params(
            strings: Vec<&str>
        ) -> Option<#path_params> {
            let mut ret_val = None;
            if let (#(Some(#vec_len_check)),*) =
            (#(strings.get(#string_parse)),*) {
                if let (#(Ok(#param_type_check)),*) = (#(#vec_len_check.parse::<>()),*) {
                    ret_val = Some((#(#param_type_check),*));
                }
            }
            ret_val
        }
    )
}

/*
pub fn _match_path_params>(
            strings: Vec<&str>,
        ) -> Option<(T0, T1, T2)> {
            let mut ret_val = None;
            if let (Some(param_str_0), Some(param_str_1), Some(param_str_2)) =
                (strings.get(0), strings.get(1), strings.get(2))
            {
                if let (Ok(param_type_0), Ok(param_type_1), Ok(param_type_2)) = (
                    param_str_0.parse::<T0>(),
                    param_str_1.parse::<T1>(),
                    param_str_2.parse::<T2>(),
                ) {
                    ret_val = Some((param_type_0, param_type_1, param_type_2));
                }
            }
            ret_val
        }
*/
