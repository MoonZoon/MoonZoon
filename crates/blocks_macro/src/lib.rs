use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, File, visit::Visit};

/*
blocks!{

    #[s_var]
    fn counter_count() -> SVar<i32> {
        log!("counter_count");
        3
    }

    #[update]
    fn set_counter_count(count: i32) {
        counter_count().set(count);
    }

    #[cmp]
    fn root<'a>() -> Column<'a> {
        col![
            main_counter(),
            counters(),
        ]
    }

    #[cmp]
    fn main_counter() -> Counter {
        ...
    }

}

// generates

fn blocks() -> Option<Box<dyn Fn() -> Box<dyn Element>>> {
    counter_count();
    Some(Box::new(move || Box::new(root()) as Box<dyn Element>))  
}
*/

#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let file: File = syn::parse2(input).unwrap();
    let mut fn_visitor = FnVisitor::default();
    fn_visitor.visit_file(&file);

    let option_root = if fn_visitor.has_root {
        quote!(Some(Box::new(move || Box::new(root()) as Box<dyn Element>)))
    } else {
        quote!(None)
    };

    let s_var_idents = fn_visitor.s_var_idents;
    let fn_blocks = quote!(
        fn blocks() -> Option<Box<dyn Fn() -> Box<dyn Element>>> {
            #( #s_var_idents(); )*
            #option_root  
        }
    );
    quote!(
        #file
        #fn_blocks
    ).into()
}

#[derive(Default)]
struct FnVisitor<'ast> {
    s_var_idents: Vec<&'ast Ident>,
    has_root: bool,
}

impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    fn visit_item_fn(&mut self, function: &'ast ItemFn) {
        let function_ident = &function.sig.ident;
        
        if let Some(first_attribute_ident) = first_attribute_ident(function) {
            if first_attribute_ident == "cmp" && function_ident == "root" {
                self.has_root = true;
                return;
            }
            if first_attribute_ident == "s_var" {
                self.s_var_idents.push(function_ident);
                return;
            }
        }

    }
}

fn first_attribute_ident(function: &ItemFn) -> Option<&Ident> {
    Some(&function
        .attrs
        .first()?
        .path
        .segments
        .first()?
        .ident)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
