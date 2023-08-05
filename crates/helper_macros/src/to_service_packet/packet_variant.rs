use std::iter::FromIterator;

use crate::to_service_packet::packet_variant::keywords::is_async;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::{Error, Fields, Ident, Index, LitBool, Path, Result, Token, Type, Variant};

#[derive(Debug)]

pub enum SubPacketOrMethod {
    SubPacket(Path),
    Method(Path),
}
mod keywords {
    syn::custom_keyword!(service_method);
    syn::custom_keyword!(from_service_variant);
    syn::custom_keyword!(sub_packet);
    syn::custom_keyword!(is_async);
    syn::custom_keyword!(skip);
}
/// EnumAttributes
pub enum EnumAttribute {
    /// service_method = "method_name"
    ServiceMethod(Path),
    /// from_service_variant = "variant_name"
    FromServiceVariant(Path),
    /// sub_packet = SubPacket
    SubPacket(Path),
    /// is_async = true or is_async or is_async = false
    IsAsync(bool),
}

impl Parse for EnumAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(keywords::service_method) {
            input.parse::<keywords::service_method>()?;
            input.parse::<syn::Token![=]>()?;
            let value = input.parse()?;
            Ok(EnumAttribute::ServiceMethod(value))
        } else if lookahead.peek(keywords::from_service_variant) {
            input.parse::<keywords::from_service_variant>()?;
            input.parse::<syn::Token![=]>()?;
            let path = input.parse()?;
            Ok(EnumAttribute::FromServiceVariant(path))
        } else if lookahead.peek(keywords::sub_packet) {
            input.parse::<keywords::sub_packet>()?;
            input.parse::<syn::Token![=]>()?;
            let path = input.parse()?;
            Ok(EnumAttribute::SubPacket(path))
        } else if lookahead.peek(keywords::is_async) {
            input.parse::<keywords::is_async>()?;
            let value = if input.peek(syn::Token![=]) {
                input.parse::<syn::Token![=]>()?;
                let lit = input.parse::<LitBool>()?;
                lit.value
            } else {
                true
            };
            Ok(EnumAttribute::IsAsync(value))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
pub struct ToServicePacketVariantAttributes {
    from_service_variant: Path,
    call: SubPacketOrMethod,
    is_async: bool,
}
impl Parse for ToServicePacketVariantAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut from_service_variant = None;
        let mut is_async = true;
        let mut sub_packet_or_method = None;
        let punctuated = input.parse_terminated(EnumAttribute::parse, Token![,])?;
        for key_optional_equals in punctuated {
            match key_optional_equals {
                EnumAttribute::ServiceMethod(service) => {
                    sub_packet_or_method = Some(SubPacketOrMethod::Method(service));
                }
                EnumAttribute::FromServiceVariant(path) => {
                    from_service_variant = Some(path);
                }

                EnumAttribute::SubPacket(sub_packet) => {
                    sub_packet_or_method = Some(SubPacketOrMethod::SubPacket(sub_packet));
                }
                EnumAttribute::IsAsync(value) => {
                    is_async = value;
                }
            }
        }

        Ok(ToServicePacketVariantAttributes {
            from_service_variant: from_service_variant.ok_or_else(|| {
                Error::new(
                    input.span(),
                    "Missing attribute: from_service_variant = <Variant Name>",
                )
            })?,
            call: sub_packet_or_method.ok_or_else(|| {
                Error::new(
                    input.span(),
                    "Missing attribute: service_method = <Method Name> or sub_packet = <SubPacket Type>",
                )
            })?,
            is_async,
        })
    }
}
#[derive(Debug)]
pub struct ToServicePacketVariant {
    pub variant: Ident,
    pub packet_type: ToServicePacketVariantAttributes,
    pub fields: Vec<ToServicePacketVariantField>,
    pub enum_name: Ident,
    pub from_service_type: Path,
    pub service_ident: Ident,
}
#[derive(Debug)]

pub struct ToServicePacketVariantField {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub is_tuple: bool,
}
impl ToServicePacketVariant {
    pub fn new(
        variant: Variant,
        enum_name: Ident,
        from_service_type: Path,
        service_ident: Ident,
    ) -> Result<Self> {
        let value = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("packet"))
            .map(|attr| attr.parse_args::<ToServicePacketVariantAttributes>())
            .ok_or(Error::new_spanned(&variant, "Missing packet attribute"))??;
        let is_tuple = if let Fields::Unnamed(_) = variant.fields {
            true
        } else {
            false
        };

        let fields: Vec<_> = variant
            .fields
            .iter()
            .map(|field| ToServicePacketVariantField {
                ident: field.ident.clone(),
                ty: field.ty.clone(),
                is_tuple: is_tuple,
            })
            .collect();
        if let SubPacketOrMethod::SubPacket(_) = &value.call {
            if fields.len() != 1 {
                return Err(Error::new_spanned(
                    &variant,
                    "SubPacket variants must have exactly one field",
                ));
            }
            if !fields.get(0).unwrap().is_tuple {
                return Err(Error::new_spanned(
                    &variant,
                    "SubPacket variants must be a tuple struct",
                ));
            }
        }

        Ok(ToServicePacketVariant {
            variant: variant.ident,
            packet_type: value,
            fields,
            enum_name,
            from_service_type,
            service_ident,
        })
    }
    pub fn token_stream_catch(mut self) -> Result<TokenStream> {
        let ToServicePacketVariant {
            variant,
            packet_type,
            mut fields,
            enum_name,
            from_service_type,
            service_ident,
        } = self;

        let mut field_names = Vec::with_capacity(fields.len());
        let mut values = Vec::with_capacity(fields.len());

        for (index, value) in fields.iter().enumerate() {
            let (field_name, value) = if let Some(ident) = value.ident.clone() {
                (
                    quote! {
                        #ident
                    },
                    ident,
                )
            } else {
                let index = Index::from(index);
                let name = format_ident!("field_{}", index);
                (
                    quote! {
                       #index: #name
                    },
                    name,
                )
            };
            field_names.push(field_name);
            values.push(value);
        }

        let response_variant = packet_type.from_service_variant;

        match packet_type.call {
            SubPacketOrMethod::SubPacket(sub) => {
                let result = quote! {
                   #enum_name::#variant {
                        #(#field_names),*
                    } => {
                        #sub::handle(#(#values),*,#service_ident).await.map(#response_variant)
                    }
                };
                Ok(result)
            }
            SubPacketOrMethod::Method(method) => {
                let result = if packet_type.is_async {
                    quote! {
                        #enum_name::#variant{
                            #(#field_names),*
                        } => {
                            #method(#service_ident, #(#values),*).await.map(#response_variant)
                        }
                    }
                } else {
                    quote! {
                        #enum_name::#variant{
                            #(#field_names),*
                        } => {
                            #method(#service_ident, #(#values),*).map(#response_variant)
                        }
                    }
                };
                Ok(result)
            }
        }
    }
}
