use proc_macro::TokenStream;
use syn::{parse_quote, spanned::Spanned, ItemFn, ReturnType, Type};

// ```
// #[static_ref]
// fn columns() -> &'static MutableVec<()> {
//     MutableVec::new_with_values(vec![(); 5])
// }
// ```
//
// generates:
//
// ```
// fn columns() -> &'static MutableVec<()> {
//     use once_cell::race::OnceBox;
//     static INSTANCE: OnceBox<MutableVec<()>> = OnceBox::new();
//     INSTANCE.get_or_init(move || Box::new(MutableVec::new_with_values(vec![(); 5])))
// }
// ```

#[proc_macro_attribute]
pub fn static_ref(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let data_type =
        data_type(&input_fn.sig.output).expect("the function has to return &'static MyType");

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({
        use once_cell::race::OnceBox;
        static INSTANCE: OnceBox<#data_type> = OnceBox::new();
        INSTANCE.get_or_init(move || Box::new(#inner_block))
    });

    quote::quote_spanned!(input_fn.span()=>
        #input_fn
    )
    .into()
}

fn data_type(return_type: &ReturnType) -> Option<&Box<Type>> {
    let type_ = match return_type {
        ReturnType::Type(_, type_) => type_,
        _ => None?,
    };
    let type_reference = match type_.as_ref() {
        Type::Reference(type_reference) => type_reference,
        _ => None?,
    };
    if type_reference.mutability.is_some() {
        None?
    }
    if type_reference.lifetime.as_ref()?.ident != "static" {
        None?
    }
    Some(&type_reference.elem)
}
