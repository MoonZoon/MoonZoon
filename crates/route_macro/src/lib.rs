use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    parse_quote, spanned::Spanned, Ident, ItemEnum, ItemImpl, Attribute, Variant, Arm, LitStr, Token, 
    punctuated::{Punctuated, Pair}, 
    parse::{self, Parse, ParseStream}};
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
//
// impl Route {
//     fn route_0_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 2 { None? }
//         if segments[0] != "report" { None? }
//         let route = Self::ReportWithFrequency { 
//             frequency: RouteSegment::from_string_segment(&segments[1])? 
//         };
//         Some(route)
//     }
//
//     fn route_1_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "report" { None? }
//         let route = Self::Report;
//         Some(route)
//     }
//
//     fn route_2_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "login" { None? }
//         let route = Self::Login;
//         Some(route)
//     }
//
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
//             Self::ReportWithFrequency { frequency } => format!(
//                 "/report/{frequency}", 
//                 frequency = encode_uri_component(frequency.into_string_segment()),
//             ).into(),
//             Self::Report => "/report".into(),
//             Self::Login => "/login".into(),
//             Self::Root => "/".into(),
//         }
//     }
//
//     fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
//         unimplemented!()
//     }
// }
// ```

struct Route<'a> {
    ident: &'a Ident,
    fields: Vec<&'a Ident>,
    segments: Vec<RouteSegment>,
}

enum RouteSegment {
    LitStr(LitStr),
    Ident(Ident),
}

impl Parse for RouteSegment {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(RouteSegment::LitStr)
        } else if lookahead.peek(Ident) {
            input.parse().map(RouteSegment::Ident)
        } else {
            Err(lookahead.error())
        }
    }
}

#[proc_macro_attribute]
pub fn route(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_enum: ItemEnum = syn::parse(input)
        .expect("'route' attribute is applicable only to enums and their variants");

    let variant_count = input_enum.variants.len();

    let mut routes = vec![
        // Route {
        //     ident: format_ident!("ReportWithFrequency"),
        //     fields: vec![format_ident!("frequency")],
        //     url_template: LitStr::new("/report/{frequency}", Span::call_site()),
        // },
        // Route {
        //     ident: format_ident!("Report"),
        //     fields: vec![],
        //     url_template: LitStr::new("/report", Span::call_site()),
        // },
        // Route {
        //     ident: format_ident!("Login"),
        //     fields: vec![],
        //     url_template: LitStr::new("/login", Span::call_site()),
        // },
        // Route {
        //     ident: format_ident!("Root"),
        //     fields: vec![],
        //     url_template: LitStr::new("/", Span::call_site()),
        // },
    ];

    for variant in &mut input_enum.variants {
        let route_attr = take_route_attr(variant);

        let fields = variant.fields.iter().map(|field| {
            field.ident.as_ref().expect("variant can contain only named fields")
        }).collect::<Vec<_>>();

        let route = Route {
            ident: &variant.ident,
            fields,
            segments: get_route_segments(&route_attr),
        };
        routes.push(route);
    }

    let impl_from_route_segments = generate_impl_from_route_segments(variant_count);
    let impl_into_cow_str = generate_impl_into_cow_str(&routes);

    quote::quote_spanned!(input_enum.span()=>
        #input_enum
        #impl_from_route_segments
        #impl_into_cow_str
    )
    .into()
}

fn take_route_attr(variant: &mut Variant) -> Attribute {
    let route_attr_index = variant.attrs.iter().position(|attr| {
        attr.path.get_ident().map(|ident| ident == "route").unwrap_or_default()
    }).expect("'route' attribute is required for all variants");
    variant.attrs.remove(route_attr_index)
}

fn get_route_segments(route_attr: &Attribute) -> Vec<RouteSegment> {
    let parser = Punctuated::<RouteSegment, Token![,]>::parse_terminated;
    route_attr.parse_args_with(parser)
        .expect("only string literals and variant field names are allowed in the 'route' attribute")
        .into_pairs()
        .map(Pair::into_value)
        .collect()
}

fn generate_impl_from_route_segments(variant_count: usize) -> ItemImpl {
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

fn generate_impl_into_cow_str(routes: &[Route]) -> ItemImpl {
    let match_arms = routes.iter().map(|route| -> Arm {
        let Route { ident, fields, segments} = route;
        let url_template = assemble_url_template(segments);

        if fields.is_empty() {
            parse_quote!(
                Self::#ident => #url_template.into()
            )
        } else {
            parse_quote!(
                Self::#ident { #(#fields),* } => format!(
                    #url_template,
                    #(#fields = encode_uri_component(#fields.into_string_segment())),*
                ).into()
            )
        }
    });

    parse_quote!(
        impl<'a> IntoCowStr<'a> for Route {
            fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
                match self {
                    #(#match_arms),*
                }
            }
            fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
                unimplemented!()
            }
        }
    )
}

fn assemble_url_template(segments: &[RouteSegment]) -> LitStr {
    let mut url_template = String::new();
    for segment in segments {
        url_template.push('/');
        match segment {
            RouteSegment::LitStr(lit_str) => {
                url_template.push_str(&lit_str.value())
            }
            RouteSegment::Ident(ident) => {
                url_template.push('{');
                url_template.push_str(&ident.to_string());
                url_template.push('}');
            }
        }
    }
    if url_template.is_empty() {
        url_template.push('/');
    }
    LitStr::new(&url_template, Span::call_site())
}
