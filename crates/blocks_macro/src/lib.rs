use proc_macro::TokenStream;
use quote::quote;
use syn;

/*
blocks!{

    #[s_var]
    fn counter_count() -> SVar<i32> {
        log!("counter_count");
        3
    }

    #[update]
    fn set_counter_count(count: i32) {
        counter_count().set(count);
    }

    #[cmp]
    fn root<'a>() -> Column<'a> {
        col![
            main_counter(),
            counters(),
        ]
    }

    #[cmp]
    fn main_counter() -> Counter {
        ...
    }

}

// generates

fn blocks() -> Option<Box<dyn Fn() -> Box<dyn Element>>> {
    counter_count();
    Some(Box::new(move || Box::new(root()) as Box<dyn Element>))  
}
*/

#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let mut input = proc_macro2::TokenStream::from(input);

    let fn_blocks = quote!(
        fn blocks() -> Option<Box<dyn Fn() -> Box<dyn Element>>> {
            counter_count();
            Some(Box::new(move || Box::new(root()) as Box<dyn Element>))  
        }
    );

    input.extend(fn_blocks);
    input.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
