use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::{LitInt, LitStr};

fn parse_int(version: Option<String>) -> LitInt {
    version
        .and_then(|s| s.parse::<i64>().ok())
        .map(|i| LitInt::new(&i.to_string(), Span::call_site()))
        .unwrap_or(LitInt::new("0", Span::call_site()))
}

pub fn current_semver() -> TokenStream {
    let major = parse_int(std::env::var("CARGO_PKG_VERSION_MAJOR").ok());
    let minor = parse_int(std::env::var("CARGO_PKG_VERSION_MINOR").ok());
    let patch = parse_int(std::env::var("CARGO_PKG_VERSION_PATCH").ok());
    let pre = std::env::var("CARGO_PKG_VERSION_PRE")
        .ok()
        .map(|s| LitStr::new(&s, Span::call_site()))
        .map(|lit| {
            quote! {
                semver::Prerelease::new(#lit).unwrap_or_default()
            }
        })
        .unwrap_or(quote! {
            semver::Prerelease::default()
        });
    let output = quote! {
        semver::Version{
            major: #major,
            minor: #minor,
            patch: #patch,
            pre: #pre,
            build: semver::BuildMetadata::default(),
        }
    };

    output.into()
}
