use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Lit, Type};

pub struct ConstAndDefaultFunction {
    pub name: syn::Ident,
    pub ty: Type,
    pub value: Lit,
}
impl Parse for ConstAndDefaultFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        input.parse::<syn::Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { name, ty, value })
    }
}
pub fn handle(input: ConstAndDefaultFunction) -> syn::Result<TokenStream> {
    let name = input.name;
    let name_for_function = Ident::new(&name.to_string().to_lowercase(), name.span().into());
    let ty = input.ty;
    let value = input.value;
    let result = quote! {
        pub const #name: #ty = #value;
        #[inline(always)]
        fn #name_for_function() -> #ty {
            #name
        }
    };
    Ok(result.into())
}
