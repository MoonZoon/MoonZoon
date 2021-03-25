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
        let component_body = c_var(move || {
            let __id = TrackedCallId::current();
            let __block = __Block::Cmp(__id);
            let __parent_call_from_macro = __TrackedCallStack::parent();
            __ComponentData {
                creator: std::rc::Rc::new(move || {
                    __BlockCallStack::push(__block);
                    // __Relations::remove_dependencies(&__block);
                    let output = (|| #inner_block)();
                    __BlockCallStack::pop();
                    let mut cmp = output.into_component();
                    cmp.component_data_id = Some(__id);
                    cmp
                }),
                parent_selected_index_from_macro: __parent_call_from_macro.as_ref().map(|parent| {
                    parent.borrow().selected_index
                }),
                parent_call_from_macro: __parent_call_from_macro,
                parent_call: None,
                parent_selected_index: None,
                rcx: None,
                children: Vec::new(),
                should_call_creator: true,
            }
        });   
        if component_body.map(|body| body.should_call_creator) {
            let creator = &component_body.inner().creator;
            component_body.update_mut(|body| { body.should_call_creator = false; });

            if let Some(__Block::Cmp(component_data_id)) = __BlockCallStack::last() {
                C_VARS.with(move |c_vars| {
                    // log!("push ComponentChild::Cmp");
                    let mut c_vars = c_vars.borrow_mut();
            
                    let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
                    component_data.children.push(ComponentChild::CmpVar(component_body.id));
            
                    c_vars.insert(component_data_id, component_data);
                });
            }

            creator()
        } else {
            Cmp {
                element: None,
                component_data_id: None,
            }
        }
    });

    quote::quote_spanned!(input_fn.span()=>
        #[tracked_call]
        #input_fn
    ).into()
}
