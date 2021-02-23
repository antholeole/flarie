extern crate flarie;
extern crate proc_macro;
extern crate syn;
extern crate proc_quote;

mod create_fn;
mod path_params;
use proc_macro::TokenStream;

use syn::{parse_macro_input, AttributeArgs, ItemFn};

macro_rules! build_methods {
    ($curr:ident, $($rest:tt)+) => {
        build_methods!($curr);
        build_methods!($($rest)*);
    };

    ($curr:ident) => {
        #[proc_macro_attribute]
        pub fn $curr(metadata: TokenStream, input: TokenStream) -> TokenStream {
            let metadata = parse_macro_input!(metadata as AttributeArgs);
            let input = parse_macro_input!(input as ItemFn);

            create_fn::parse_route_fn(input, metadata, $curr)
                .unwrap_or_else(|err| err.to_compile_error())
                .into()
        }
    };
}

build_methods!(get, post, delete, put, patch);
