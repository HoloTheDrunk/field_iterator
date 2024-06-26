use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, Data, DeriveInput, Fields},
};

#[proc_macro_derive(Iterable)]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let fields_iter = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_name = field_ident.as_ref().unwrap().to_string();
        quote! {
            (#field_name, &(self.#field_ident) as &dyn ::std::any::Any)
        }
    });

    let fields_iter_mut = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let field_name = field_ident.as_ref().unwrap().to_string();
        quote! {
            (#field_name, &mut (self.#field_ident) as &mut dyn ::std::any::Any)
        }
    });

    quote! {
        impl ::field_iterator::Iterable for #struct_name {
            fn iter<'a>(&'a self) -> ::std::vec::IntoIter<(&'static str, &'a dyn std::any::Any)> {
                vec![
                    #(#fields_iter),*
                ].into_iter()
            }

            fn iter_mut<'a>(&'a mut self) -> ::std::vec::IntoIter<(&'static str, &'a mut dyn std::any::Any)> {
                vec![
                    #(#fields_iter_mut),*
                ].into_iter()
            }
        }
    }.into()
}
