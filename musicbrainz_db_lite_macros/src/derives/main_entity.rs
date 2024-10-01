use proc_macro::TokenStream;
use syn::Data;
use crate::database_atributes::DatabaseAtribute;
use darling::FromDeriveInput;
use quote::quote;
use crate::derives::methods::update_date::impl_update_date;

pub fn derive_main_entity_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let args = match DatabaseAtribute::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();

            let full_update = impl_update_date(&struct_identifier, &args.table, &args.primary_key);

            quote! {
                #[automatically_derived]
                impl #struct_identifier {
                    #full_update
                }
            }
        }
        _ => unimplemented!()
    }.into()
}