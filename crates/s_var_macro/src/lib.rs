use proc_macro::TokenStream;

/* 
#[s_var]
fn counter_count() -> i32 {
    3
}

// to 

fn counter_count() -> SVar<i32> {
    s_var("counter_count", || {
        3
    })
}

*/
#[proc_macro_attribute]
pub fn s_var(_attr: TokenStream, input: TokenStream) -> TokenStream {

    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
