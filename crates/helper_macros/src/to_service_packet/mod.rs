use std::iter::FromIterator;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::{DeriveInput, Error, Path, Result, Token};

use crate::to_service_packet::packet_variant::ToServicePacketVariant;

mod packet_variant;

mod keywords {
    syn::custom_keyword!(service_type);
    syn::custom_keyword!(from_service_type);
}
enum KeywordEqualsType {
    ServiceType(Path),
    FromServiceVariant(Path),
}
impl Parse for KeywordEqualsType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keywords::service_type) {
            input.parse::<keywords::service_type>()?;
            input.parse::<Token![=]>()?;
            let lit = input.parse()?;
            Ok(KeywordEqualsType::ServiceType(lit))
        } else if lookahead.peek(keywords::from_service_type) {
            input.parse::<keywords::from_service_type>()?;
            input.parse::<Token![=]>()?;
            let lit = input.parse()?;
            Ok(KeywordEqualsType::FromServiceVariant(lit))
        } else {
            Err(input.error("expected enum attribute"))
        }
    }
}

pub struct ToServicePacketEnumAttribute {
    pub service_type: Path,
    pub from_service_type: Path,
}
impl Parse for ToServicePacketEnumAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        /// service_type = Type
        let punctuated = input.parse_terminated(KeywordEqualsType::parse, Token![,])?;

        let mut service_type: Option<Path> = None;
        let mut from_service_type: Option<Path> = None;

        for key_optional_equals in punctuated {
            match key_optional_equals {
                KeywordEqualsType::ServiceType(ty) => {
                    service_type = Some(ty);
                }
                KeywordEqualsType::FromServiceVariant(ty) => {
                    from_service_type = Some(ty);
                }
            }
        }
        Ok(ToServicePacketEnumAttribute {
            service_type: service_type.ok_or_else(|| {
                Error::new(
                    input.span(),
                    "Missing attribute: service_type = <Service Type>",
                )
            })?,
            from_service_type: from_service_type.ok_or_else(|| {
                Error::new(
                    input.span(),
                    "Missing attribute: from_service_type = <Service Type>",
                )
            })?,
        })
    }
}

pub fn handle(input: DeriveInput) -> Result<TokenStream> {
    // Get service_type attribute on the enum level
    let span = input.span();

    let name = input.ident.clone();
    let ToServicePacketEnumAttribute {
        service_type,
        from_service_type,
    } = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("service_packet"))
        .map(|attr| attr.parse_args::<ToServicePacketEnumAttribute>())
        .ok_or(Error::new_spanned(
            &input,
            "Missing service_packet attribute",
        ))??;

    let enum_input = match input.data {
        syn::Data::Enum(enum_input) => enum_input,
        _ => {
            unreachable!()
        }
    };
    let service_name = format_ident!("service");
    let mut variants = Vec::with_capacity(enum_input.variants.len());
    for variant in enum_input.variants {
        let variant1 = ToServicePacketVariant::new(
            variant,
            input.ident.clone(),
            from_service_type.clone(),
            service_name.clone(),
        )?;
        let variant_token_stream = variant1.token_stream_catch()?;
        variants.push(variant_token_stream);
    }

    let result = quote! {
        impl #name {
           pub async fn handle<S: #service_type + utils::service::Service>(self, #service_name: &S) -> Result<#from_service_type, S::ServiceError> {
                match self {
                    #(#variants)*
                }
            }
        }
    };
    Ok(result)
}
