use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn update(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
