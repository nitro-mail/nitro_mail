use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

mod const_and_default_function;
mod current_semver;
mod to_service_packet;

/// Creates the semver::Version Object without having to parse it at runtime
#[proc_macro]
pub fn current_semver(_input: TokenStream) -> TokenStream {
    current_semver::current_semver()
}
///
/// # Example
/// ```rust
/// use helper_macros::const_and_default_function;
/// const_and_default_function!(DEFAULT_FOO: i32 = 42);
/// const_and_default_function!(DEFAULT_BAR: i32 = 42);
/// const_and_default_function!(DEFAULT_BAZ: i32 = 42);
/// ```
///
/// # Output
/// ```rust
/// use helper_macros::const_and_default_function;
///
/// const DEFAULT_FOO: i32 = 42;
///  const DEFAULT_BAR: i32 = 42;
/// const DEFAULT_BAZ: i32 = 42;
/// fn default_foo() -> i32 {
///    DEFAULT_FOO
/// }
/// fn default_bar() -> i32 {
///   DEFAULT_BAR
/// }
/// fn default_baz() -> i32 {
///   DEFAULT_BAZ
/// }
/// ```
///
#[proc_macro]
pub fn const_and_default_function(input: TokenStream) -> TokenStream {
    match const_and_default_function::handle(parse_macro_input!(
        input as const_and_default_function::ConstAndDefaultFunction
    )) {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
#[proc_macro_derive(ToServicePacket, attributes(service_packet, packet))]
pub fn to_service_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Check if its an enum
    if let syn::Data::Enum(_) = &input.data {
        match to_service_packet::handle(input) {
            Ok(ok) => ok.into(),
            Err(err) => err.to_compile_error().into(),
        }
    } else {
        syn::Error::new_spanned(input, "ToServicePacket can only be derived for enums")
            .to_compile_error()
            .into()
    }
}
