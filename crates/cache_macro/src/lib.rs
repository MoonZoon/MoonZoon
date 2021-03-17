use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn, ReturnType, Type};
use uuid::Uuid;

/* 
#[cache]
fn counter_count() -> i32 {
    3
}

// or 

#[cache]
fn counter_count() -> Cache<i32> {
    3
}

// to 

fn counter_count() -> Cache<i32> {
    const __ID: u128 = 0x936DA01F9ABD4D9D80C702AF85C822A8;
    const __BLOCK: __Block = __Block::Cache(__ID);
    __Relations::add_dependency(__BLOCK);
    cache(__ID, || {
        __BlockCallStack::push(__BLOCK);
        __Relations::remove_dependencies(__BLOCK);
        let output = (|| {
            3
        })();
        __BlockCallStack::pop();
        output
    })
}
*/
#[proc_macro_attribute]
pub fn cache(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let id = Uuid::new_v4().as_u128();

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        const __ID: u128 = #id;
        const __BLOCK: __Block = __Block::Cache(__ID);
        __Relations::add_dependency(__BLOCK);
        cache(__ID, || {
            __BlockCallStack::push(__BLOCK);
            __Relations::remove_dependencies(&__BLOCK);
            let output = (|| #inner_block)();
            __BlockCallStack::pop();
            output
        })
    });

    input_fn.sig.output = match input_fn.sig.output {
        ReturnType::Default => {
            parse_quote!(-> Cache<()>)
        },
        ReturnType::Type(_, type_) if is_first_segment_cache(&type_)  => {
            parse_quote!(-> #type_)
        },
        ReturnType::Type(_, type_) => {
            parse_quote!(-> Cache<#type_>)
        },
    };

    input_fn.into_token_stream().into()
}

fn is_first_segment_cache(type_: &Box<Type>) -> bool {
    if let Type::Path(type_path) = type_.as_ref() {
        type_path
            .path
            .segments
            .first()
            .map(|path_segment| path_segment.ident == "Cache")
            .unwrap_or_default()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
