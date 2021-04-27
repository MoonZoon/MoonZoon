use proc_macro::TokenStream;
use syn::{parse_quote, spanned::Spanned, ItemFn, parse_macro_input, AttributeArgs, Expr, NestedMeta, Lit, Meta};

#[proc_macro_attribute]
pub fn tracked_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    
    let key: Expr = if let Some(key) = key_from_args(&attr_args) {
        parse_quote!({
            use std::hash::{Hasher, Hash};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            (#key).hash(&mut hasher);
            Some(hasher.finish())
        })
    } else {
        parse_quote!({ None })
    };

    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        let __tracked_call = __TrackedCall::create(#key);
        // log!("from macro: {:#?}", __tracked_call_id);
        __TrackedCallStack::push(std::rc::Rc::new(std::cell::RefCell::new(__tracked_call))); 
        let output = (move || #inner_block)();
        __TrackedCallStack::pop();
        output
    });

    quote::quote_spanned!(input_fn.span()=>
        #input_fn
    ).into()
}

fn key_from_args(args: &[NestedMeta]) -> Option<Expr> {
    assert!(args.len() <= 1);

    args.get(0).map(|arg| match arg {
        NestedMeta::Meta(Meta::NameValue(meta_name_value)) => {
            assert!(
                meta_name_value.path.is_ident("key"),
                "only `key = \"...\" argument is supported"
            );

            match &meta_name_value.lit {
                Lit::Str(lit_str) => lit_str.parse().unwrap(),
                _ => panic!("`key` argument accepts a string literal"),
            }
        }
        _ => panic!("only `key = \"...\" argument is supported"),
    })
}
