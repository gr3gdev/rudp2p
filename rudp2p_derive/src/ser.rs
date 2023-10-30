use proc_macro::TokenStream;
use quote::quote;

fn serialize_struct(struct_name: &syn::Ident, s: &syn::DataStruct) -> TokenStream {
    let body = s.fields.iter().map(|f| serialize_field(f));
    TokenStream::from(quote! {
        impl serialize_bits::ser::SerializerData for #struct_name {
            fn to_data(&self) -> Vec<u8> {
                let mut res = Vec::new();
                #(#body)*
                res
            }
        }
    })
}

pub(crate) fn serialize_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let name = &field.ident;
    quote! {
        res.append(&mut self.#name.to_data());
    }
}

pub(crate) fn serialize_data(name: &syn::Ident, data: &syn::Data) -> TokenStream {
    match &data {
        syn::Data::Struct(s) => serialize_struct(&name, s),
        syn::Data::Enum(_) => panic!("Enum {name} must implement SerializerData trait"),
        syn::Data::Union(_) => panic!("Union {name} must implement SerializerData trait"),
    }
}
