use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn call_tree(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
