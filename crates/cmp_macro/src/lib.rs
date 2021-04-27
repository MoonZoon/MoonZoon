use proc_macro::{TokenStream, TokenTree, Group, Delimiter};
use syn::{parse_quote, ItemFn, FnArg, Pat, spanned::Spanned, Attribute};

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
pub fn cmp(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_fn: ItemFn = syn::parse(input).unwrap();

    let fn_arg_idents = input_fn.sig.inputs.iter().filter_map(|fn_arg| {
        if let FnArg::Typed(pat_type) = fn_arg {
            match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) => return Some(pat_ident.ident.clone()),
                _ => return None
            }
        } 
        None
    });

    let inner_block = input_fn.block;
    input_fn.block = parse_quote!({ 
        let component_body = c_var(move || {
            let __id = TrackedCallId::current();
            let __block = __Block::Cmp(__id);
            let __parent_call_from_macro = __TrackedCallStack::parent();
            __ComponentData {
                id: __id,
                creator: std::rc::Rc::new(move || {
                    __BlockCallStack::push(__block);
                    // __Relations::remove_dependencies(&__block);

                    #( #fn_arg_idents.add_dependency(); )*

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
                previous_children: std::collections::HashSet::new(),
                children: std::collections::HashSet::new(),
                should_call_creator: true,
            }
        });   

        if let Some(__Block::Cmp(component_data_id)) = __BlockCallStack::last() {
            C_VARS.with(move |c_vars| {
                // log!("push ComponentChild::Cmp");
                let mut c_vars = c_vars.borrow_mut();
        
                let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
                component_data.children.insert(ComponentChild::Cmp(component_body.id));
        
                c_vars.insert(component_data_id, component_data);
            });
        }

        if component_body.map(|body| body.should_call_creator) {
            component_body.update_mut(|body| { body.should_call_creator = false; });
            let creator = component_body.map(|body| body.creator.clone());
            creator()
        } else {
            Cmp {
                element: None,
                component_data_id: None,
            }
        }
    });

    let mut tracked_call_attribute: Attribute = parse_quote!(#[tracked_call]);
    tracked_call_attribute.tokens = TokenStream::from(
        TokenTree::Group(Group::new(Delimiter::Parenthesis, args))
    ).into();

    quote::quote_spanned!(input_fn.span()=>
        #tracked_call_attribute
        #input_fn
    ).into()
}
