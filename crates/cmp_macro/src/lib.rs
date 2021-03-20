use proc_macro::TokenStream;
use syn::{parse_quote, ItemFn, spanned::Spanned};

/* 
#[cmp]
fn root<'a>() -> Cmp<'a> {
    col![
        control_counters(),
        counters(),
    ]
}

// to 

fn root<'a>() -> Cmp<'a> {
    let creator = l_var(move || (move || {
        col![
            control_counters(),
            counters(),
        ]
    }));
    creator.inner()().into_component()
}
*/
#[proc_macro_attribute]
pub fn cmp(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        let creator = l_var(move || (move || #inner_block));
        creator.inner()().into_component()
    });

    quote::quote_spanned!(input_fn.span()=>
        #[tracked_call]
        #input_fn
    ).into()
}
