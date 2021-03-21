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
    let component_body = l_var(move || {
        let __id = TrackedCallId::current();
        let __block = __Block::Cmp(__id);
        __ComponentData {
            creator: std::rc::Rc::new(move || {
                __BlockCallStack::push(__block);
                __Relations::remove_dependencies(&__block);
                let output = (|| {
                    col![
                        control_counters(),
                        counters(),
                    ]
                })();
                __BlockCallStack::pop();
                output.into_component()
            }),
        }
    });   
    let creator = &component_body.inner().creator;
    creator()
}
*/
#[proc_macro_attribute]
pub fn cmp(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        let component_body = l_var(move || {
            let __id = TrackedCallId::current();
            let __block = __Block::Cmp(__id);
            __ComponentData {
                creator: std::rc::Rc::new(move || {
                    __BlockCallStack::push(__block);
                    __Relations::remove_dependencies(&__block);
                    let output = (|| #inner_block)();
                    __BlockCallStack::pop();
                    let mut cmp = output.into_component();
                    cmp.component_data_id = Some(__id);
                    cmp
                }),
                context: None,
            }
        });   
        let creator = &component_body.inner().creator;
        creator()
    });

    quote::quote_spanned!(input_fn.span()=>
        #[tracked_call]
        #input_fn
    ).into()
}
