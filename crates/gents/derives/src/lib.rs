use syn::{parse_macro_input, DeriveInput};
mod container;

use proc_macro::TokenStream;

#[proc_macro_derive(TS, attributes(ts))]
pub fn derive_ts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    get_impl_block(input).into()
}

fn get_impl_block(input: DeriveInput) -> proc_macro2::TokenStream {
    todo!()
}
