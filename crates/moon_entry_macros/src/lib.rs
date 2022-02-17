use proc_macro::TokenStream;
use quote::quote;

/// Marks async main function as the Moon system entry-point.

/// # Examples
/// ```
/// #[moon::main]
/// async fn main() {
///     async { println!("Hello world"); }.await
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut output: TokenStream = (quote! {
        #[moon::actix_web::rt::main(system = "moon::actix_web::rt::System")]
    })
    .into();

    output.extend(item);
    output
}

/// Marks async test functions to use the Moon system entry-point.
///
/// # Examples
/// ```
/// #[moon::test]
/// async fn test() {
///     assert_eq!(async { "Hello world" }.await, "Hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut output: TokenStream = (quote! {
        #[moon::actix_web::rt::test(system = "moon::actix_web::rt::System")]
    })
    .into();

    output.extend(item);
    output
}
