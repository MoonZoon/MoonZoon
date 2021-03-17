use proc_macro::TokenStream;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, AttributeArgs, Expr, ItemFn, Lit, Meta,
    NestedMeta,
};

#[proc_macro_attribute]
pub fn call_tree(args: TokenStream, input: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(args);
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let inner_block = input_fn.block;
    input_fn.block = if let Some(slot_expr) = slot_from_args(&args) {
        parse_quote! {{
            CallTree::call_in_slot(#slot_expr, move || #inner_block)
        }}
    } else {
        parse_quote! {{ CallTree::call(move || #inner_block) }}
    };

    quote::quote_spanned!(input_fn.span()=>
        #[track_caller]
        #input_fn
    )
    .into()
}

/// parse the attribute arguments, retrieving an an expression to use as part of
/// the slot
fn slot_from_args(args: &[NestedMeta]) -> Option<Expr> {
    assert!(args.len() <= 1);

    args.get(0).map(|arg| match arg {
        NestedMeta::Meta(Meta::NameValue(kv)) => {
            assert!(
                kv.path.is_ident("slot"),
                "only `slot = \"...\" argument is supported by #[call_tree]"
            );

            match &kv.lit {
                Lit::Str(l) => l.parse().unwrap(),
                _ => panic!("`slot` argument accepts a string literal"),
            }
        }
        _ => panic!("only `slot = \"...\" argument is supported by #[call_tree]"),
    })
}

// NOTE: Heavily inspired by the crate topo
