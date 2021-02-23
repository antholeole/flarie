use proc_macro2::{TokenStream, Literal};
use proc_quote::quote;
use quote::format_ident;

pub fn find_path_type<'a>(
    args: &'a syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> (Vec<&'a syn::Ident>, Vec<&'a syn::Ident>, bool) {
    let mut types = Vec::new();
    let mut names = Vec::new();
    let mut get_names = false;

    args.iter().for_each(|v| {
        if let syn::FnArg::Typed(pat) = v {
            if let syn::Type::Path(p) = &*pat.ty {
                p.path.segments.iter().for_each(|path_seg| {
                    if path_seg.ident == "Path" {
                        if let syn::PathArguments::AngleBracketed(ab) = &path_seg.arguments {
                            ab.args.iter().for_each(|arg| {
                                if let syn::GenericArgument::Type(syn::Type::Tuple(type_tuple)) =
                                    arg
                                {
                                    type_tuple.elems.iter().for_each(|el| {
                                        if let syn::Type::Path(path) = el {
                                            path.path.segments.iter().for_each(|path_seg| {
                                                types.push(&path_seg.ident);
                                                get_names = true;
                                            });
                                        }
                                    });
                                }
                            })
                        }
                    }
                })
            }

            if get_names {
                if let syn::Pat::Tuple(tuple) = &*pat.pat {
                    tuple.elems.iter().for_each(|el| {
                        if let syn::Pat::Ident(pat_ident) = el {
                            names.push(&pat_ident.ident);
                        }
                    });
                }
            }
        }
    });

    (names, types, false)
}


pub(crate) fn generate_match_no_path_params() -> TokenStream {
    quote!(
        fn _match_path_params(strings: Vec<&str>) -> Option<()> {
            Some(())
        }
    )
}

pub (crate) fn generate_match_n_path_params_fn(
    path_params: Vec<&syn::Ident>
) -> TokenStream {
    let vec_len_check = (0usize..(path_params.len())).map(|v| format_ident!("param_str_{}", v)).collect::<Vec::<syn::Ident>>();
    let string_parse = (0usize..path_params.len()).map(Literal::usize_unsuffixed);
    let param_type_check = (0usize..path_params.len()).map(|v| format_ident!("param_type_{}", v)).collect::<Vec::<syn::Ident>>();

    quote!(
        fn _match_path_params(
            strings: Vec<&str>
        ) -> Option<(#(#path_params),*)> {
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

pub (crate) fn generate_param_data_extractor(names: Vec<&'_ syn::Ident>, is_bundeled: bool) -> TokenStream {
    if names.is_empty() {
        quote!()
    } else if is_bundeled {
        let name = names.get(0).unwrap();
        quote!(let #name = __data.path_params.unwrap();)
    } else {
        quote!(let (#(#names),*) = __data.path_params.unwrap();)
    }   
}