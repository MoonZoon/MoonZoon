use proc_macro::TokenStream;
use syn::{parse_quote, spanned::Spanned, ItemFn};

#[proc_macro_attribute]
pub fn tracked_call(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        let __tracked_call_id = __TrackedCallId::get_or_create();
        __TrackedCallStack::push(__tracked_call_id); 
        let output = (move || #inner_block)();
        __TrackedCallStack::pop();
        output
    });

    quote::quote_spanned!(input_fn.span()=>
        #[track_caller]
        #input_fn
    )
    .into()
}
