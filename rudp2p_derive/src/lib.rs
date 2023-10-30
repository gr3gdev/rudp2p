extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

pub(crate) mod des;
pub(crate) mod ser;

#[proc_macro_derive(DeserializeData)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    des::deserialize_data(&ast.ident, &ast.data)
}

#[proc_macro_derive(SerializeData)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    ser::serialize_data(&ast.ident, &ast.data)
}
