use proc_macro::TokenStream;

#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let _ = input;
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
