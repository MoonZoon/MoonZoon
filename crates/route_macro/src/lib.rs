use proc_macro::TokenStream;
use syn::{parse_quote, spanned::Spanned, Ident, ItemEnum, ItemImpl, Attribute, Variant};
use quote::format_ident;

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
//
// impl FromRouteSegments for Route {
//     fn from_route_segments(segments: Vec<String>) -> Option<Self> {
//         let route_fns = [
//             Self::route_0_from_route_segments,
//             Self::route_1_from_route_segments,
//             Self::route_2_from_route_segments,
//             Self::route_3_from_route_segments,
//         ];
//         for route_fn in route_fns {
//             let this = route_fn(&segments);
//             if this.is_some() {
//                 return this
//             }
//         }
//         None
//     }
// }
//
// impl<'a> IntoCowStr<'a> for Route {
//     fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
//         match self {
//             Self::ReportWithFrequency { frequency } => {
//                 format!(
//                     "/report/{}", 
//                     encode_uri_component(frequency.into_string_segment()),
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
    let mut input_enum: ItemEnum = syn::parse(input)
        .expect("'route' attribute is applicable only to enums and their variants");

    let variant_count = input_enum.variants.len();

    for variant in &mut input_enum.variants {
        let route_attr = get_route_attr(variant);
    }

    let from_route_segments = generate_from_route_segments(variant_count);

    quote::quote_spanned!(input_enum.span()=>
        #input_enum
        #from_route_segments
    )
    .into()
}

fn get_route_attr(variant: &mut Variant) -> Attribute {
    let route_attr_index = variant.attrs.iter().position(|attr| {
        attr.path.get_ident().map(|ident| ident == "route").unwrap_or_default()
    }).expect("'route' attribute is required for all variants");
    variant.attrs.remove(route_attr_index)
}

fn generate_from_route_segments(variant_count: usize) -> ItemImpl {
    let route_fn_idents = (0..variant_count).map(|index| {
        format_ident!("route_{}_from_route_segments", index)
    }).collect::<Vec<_>>();

    parse_quote!(
        impl FromRouteSegments for Route {
            fn from_route_segments(segments: Vec<String>) -> Option<Self> {
                let route_fns = [
                    #(Self::#route_fn_idents),*
                ];
                for route_fn in route_fns {
                    let this = route_fn(&segments);
                    if this.is_some() {
                        return this
                    }
                }
                None
            }
        }
    )
}
