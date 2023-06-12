mod args;
mod generate;
mod impl_info;
mod lua_ffi;
mod method_info;
mod parse;
mod util;

use crate::args::AttrArgs;
use crate::generate::generate;
use crate::parse::Item;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn luajit_ffi(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr_args as AttrArgs);
    let item = parse_macro_input!(input as Item);

    let item = generate(item, args);

    quote!(#item).into()
}