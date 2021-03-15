use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn, LitStr, ReturnType, Type};

/* 
#[s_var]
fn counter_count() -> i32 {
    3
}

// or 

#[s_var]
fn counter_count() -> SVar<i32> {
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
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let input_fn_name = &input_fn.sig.ident;
    let id = LitStr::new(&input_fn_name.to_string(), input_fn_name.span());

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        s_var(#id, || {
            #inner_block
        })
    });

    input_fn.sig.output = match input_fn.sig.output {
        ReturnType::Default => {
            parse_quote!(-> SVar<()>)
        },
        ReturnType::Type(_, type_) if is_first_segment_s_var(&type_)  => {
            parse_quote!(-> #type_)
        },
        ReturnType::Type(_, type_) => {
            parse_quote!(-> SVar<#type_>)
        },
    };

    input_fn.into_token_stream().into()
}

fn is_first_segment_s_var(type_: &Box<Type>) -> bool {
    if let Type::Path(type_path) = type_.as_ref() {
        type_path
            .path
            .segments
            .first()
            .map(|path_segment| path_segment.ident == "SVar")
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
