extern crate flarie;
extern crate proc_macro;
extern crate syn;
extern crate proc_quote;

mod create_fn;
mod path_params;
use proc_macro::TokenStream;

use syn::{parse_macro_input, AttributeArgs, ItemFn};
\
