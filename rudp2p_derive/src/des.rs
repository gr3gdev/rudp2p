use proc_macro::TokenStream;
use quote::quote;

fn deserialize_struct(struct_name: &syn::Ident, s: &syn::DataStruct) -> TokenStream {
    let body = s.fields.iter().map(|f| deserialize_field(f));
    let fields = s.fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name,
        }
    });
    TokenStream::from(quote! {
        impl serialize_bits::des::DeserializerData for #struct_name {
            fn from_data(data: &Vec<u8>, index: usize) -> (Self, usize) where Self: Sized {
                #(#body)*
                (
                    Self {
                        #(#fields)*
                    },
                    index,
                )
            }
        }
    })
}

pub(crate) fn deserialize_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let name = &field.ident;
    let t = &field.ty;
    let s_type = match t {
        syn::Type::Path(p) => p.path.segments.iter().map(|s| {
            let type_name = &s.ident;
            quote! {
                let (#name, index) = #type_name::from_data(data, index);
            }
        }),
        _ => panic!("Type not supported !"),
    };
    quote! {
        #(#s_type)*
    }
}

pub(crate) fn deserialize_data(name: &syn::Ident, data: &syn::Data) -> TokenStream {
    match &data {
        syn::Data::Struct(s) => deserialize_struct(&name, s),
        syn::Data::Enum(_) => panic!("Enum {name} must implement DeserializerData trait"),
        syn::Data::Union(_) => panic!("Union {name} must implement DeserializerData trait"),
    }
}
