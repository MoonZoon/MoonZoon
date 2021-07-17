use proc_macro::TokenStream;
use syn::{parse_quote, spanned::Spanned, ItemFn, ReturnType, Type};

// ```
// #[route]
// pub enum Route {
//     #[route("report", frequency)]
//     ReportWithFrequency { frequency: report_page::Frequency },
//     #[route("report")]
//     Report,
//     #[route("login")]
//     Login,
//     #[route()]
//     Root,
// }
// ```
//
// generates:
//
// ```
// pub enum Route {
//     ReportWithFrequency { frequency: report_page::Frequency },
//     Report,
//     Login,
//     Root,
// }

// impl Route {
//     fn route_0_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 2 { None? }
//         if segments[0] != "report" { None? }
//         let route = Self::ReportWithFrequency { 
//             frequency: RouteSegment::from_string_segment(&segments[1])? 
//         };
//         Some(route)
//     }

//     fn route_1_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "report" { None? }
//         let route = Self::Report;
//         Some(route)
//     }

//     fn route_2_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "login" { None? }
//         let route = Self::Login;
//         Some(route)
//     }

//     fn route_3_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 0 { None? }
//         let route = Self::Root;
//         Some(route)
//     }
// }

// impl FromRouteSegments for Route {
//     fn from_route_segments(segments: Vec<String>) -> Option<Self> {
//         None
//             .or_else(|| Self::route_0_from_route_segments(&segments))
//             .or_else(|| Self::route_1_from_route_segments(&segments))
//             .or_else(|| Self::route_2_from_route_segments(&segments))
//             .or_else(|| Self::route_3_from_route_segments(&segments))
//     }
// }

// impl<'a> IntoCowStr<'a> for Route {
//     fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
//         match self {
//             Self::ReportWithFrequency { frequency } => {
//                 format!(
//                     "/report/{}", 
//                     zoon::encode_uri_component(frequency.into_string_segment()),
//                 ).into()
//             }
//             Self::Report => "/report".into(),
//             Self::Login => "/login".into(),
//             Self::Root => "/".into(),
//         }
//     }

//     fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
//         unimplemented!()
//     }
// }
// ```

#[proc_macro_attribute]
pub fn route(_args: TokenStream, input: TokenStream) -> TokenStream {
    // let mut input_fn: ItemFn = syn::parse(input).unwrap();

    // let data_type =
    //     data_type(&input_fn.sig.output).expect("the function has to return &'static MyType");

    // let inner_block = input_fn.block;
    // input_fn.block = parse_quote!({
    //     use once_cell::race::OnceBox;
    //     static INSTANCE: OnceBox<#data_type> = OnceBox::new();
    //     INSTANCE.get_or_init(move || Box::new(#inner_block))
    // });

    // quote::quote_spanned!(input_fn.span()=>
    //     #input_fn
    // )
    // .into()
 
    input
}

// fn data_type(return_type: &ReturnType) -> Option<&Box<Type>> {
//     let type_ = match return_type {
//         ReturnType::Type(_, type_) => type_,
//         _ => None?,
//     };
//     let type_reference = match type_.as_ref() {
//         Type::Reference(type_reference) => type_reference,
//         _ => None?,
//     };
//     if type_reference.mutability.is_some() {
//         None?
//     }
//     if type_reference.lifetime.as_ref()?.ident != "static" {
//         None?
//     }
//     Some(&type_reference.elem)
// }
