extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_roids::{namespace_parameters, DeriveInputStructExt, FieldExt};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_derive(FieldExtract, attributes(descriptor))]
pub fn derive_field(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let fields = input.fields();
    let fields_count = fields.iter().count();
    let struct_name = input.ident.clone();

    let mut names = Vec::new();
    let mut types = Vec::new();
    let mut indexs = Vec::new();

    for (index, field) in fields.iter().enumerate() {
        names.push(field.ident.clone());
        types.push(field.type_name());
        indexs.push(index);
    }

    let params = namespace_parameters(&input.attrs, &parse_quote!(descriptor));
    let set_desc = &params[0];
    let data_desc = &params[1];

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #struct_name {
            pub const OFFSETS: &'static [usize ; #fields_count] = &{
                let sizes = [
                    #(
                        std::mem::size_of::<#types>()
                    ,)*
                ];

                let mut out = [0 ; #fields_count];
                let mut index = 1;
                while index < #fields_count {
                   out[index] = out[index - 1] + sizes[index - 1];
                    index+=1;
                }
                out
            };


            pub fn new(field: &packet::RawField) -> anyhow::Result<Self> {
                Ok(Self {
                    #(
                        #names: field.extract::<#types>(#struct_name::OFFSETS[#indexs])?
                    ,)*
                })
            }
        }

        impl std::convert::TryFrom<packet::RawField> for #struct_name {
            type Error = anyhow::Error;

            fn try_from(field: packet::RawField) -> anyhow::Result<Self> {
                Self::new(&field)
            }
        }

        impl packet::Field for #struct_name {
            const SET_DESCRIPTOR: u8 = #set_desc;
            const DATA_DESCRIPTOR: u8 = #data_desc;
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(DataPacket)]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let fields = input.fields();
    let struct_name = input.ident.clone();

    let mut names = vec![];
    let mut inner_types = vec![];

    for field in fields.iter() {
        names.push(&field.ident);

        if let syn::Type::Path(tp) = &field.ty {
            if let syn::PathArguments::AngleBracketed(ab) = &tp.path.segments[0].arguments {
                inner_types.push(ab.args[0].clone());
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn from_vec(fields: &Vec<RawField>) -> Self {
                let mut field_map = std::collections::HashMap::new();

                for field in fields {
                    field_map.insert(field.descriptor, field);
                }

                Self {
                    #(
                        #names: field_map.get(&#inner_types::DATA_DESCRIPTOR).and_then(|f| #inner_types::new(f).ok())
                    ,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
