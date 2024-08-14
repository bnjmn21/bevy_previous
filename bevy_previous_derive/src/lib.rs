use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    punctuated::{Pair, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Error, Fields, GenericParam,
};

macro_rules! err {
    ($span:expr=>$message:expr) => {
        Error::new_spanned($span, $message)
            .to_compile_error()
            .into()
    };
}

/// Derive macro for generating a `DefaultSchedule` implementation.
///
/// Only works on unit structs.
#[proc_macro_derive(DefaultSchedule)]
pub fn default_schedule(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match input.data {
        Data::Struct(DataStruct {
            struct_token: _,
            semi_token: _,
            fields: Fields::Unit,
        }) => {
            let ident = input.ident;
            if input.generics.params.is_empty() {
                quote! {
                    impl ::bevy_previous::DefaultSchedule for #ident {
                        fn default() -> Self {
                            #ident
                        }
                    }
                }
            } else {
                let params = input.generics.params;
                let unbounded_params_iter =
                    params.pairs().map(Pair::into_tuple).map(|(g, p)| match g {
                        GenericParam::Type(t) => {
                            let mut unbounded = t.clone();
                            unbounded.bounds.clear();
                            unbounded.attrs.clear();
                            (GenericParam::Type(unbounded), p.cloned())
                        }
                        GenericParam::Lifetime(l) => {
                            let mut unbounded = l.clone();
                            unbounded.bounds.clear();
                            unbounded.attrs.clear();
                            (GenericParam::Lifetime(unbounded), p.cloned())
                        }
                        GenericParam::Const(c) => {
                            let mut unbounded = c.clone();
                            unbounded.attrs.clear();
                            (GenericParam::Const(unbounded), p.cloned())
                        }
                    });
                let mut unbounded_params: Punctuated<GenericParam, Comma> = Punctuated::new();
                for (g, p) in unbounded_params_iter {
                    unbounded_params.push_value(g);
                    if let Some(p) = p {
                        unbounded_params.push_punct(p);
                    }
                }
                let where_clause = input.generics.where_clause;
                if let Some(where_clause) = where_clause {
                    quote! {
                        impl<#params> ::bevy_previous::DefaultSchedule for #ident<#unbounded_params>
                        #where_clause {
                            fn default() -> Self {
                                #ident
                            }
                        }
                    }
                } else {
                    quote! {
                        impl<#params> ::bevy_previous::DefaultSchedule for #ident<#unbounded_params> {
                            fn default() -> Self {
                                #ident
                            }
                        }
                    }
                }
            }
            .into()
        }
        _ => err!(input => "The macro only works on unit structs."),
    }
}
