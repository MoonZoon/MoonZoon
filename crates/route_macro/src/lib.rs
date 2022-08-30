use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::format_ident;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_quote,
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    Arm, Attribute, ExprIf, FieldValue, Ident, ItemEnum, ItemFn, ItemImpl, LitStr, Token, Variant,
};
use urlencoding::encode as url_encode;

// @TODO rewrite panics/expects/unwraps to `syn::Error`s (see hsluv_macro)?

// @TODO replace the compiler error `named argument never used` with
// the info what route struct field is not used in the route path

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
//         Some(Self::ReportWithFrequency {
//             frequency: RouteSegment::from_string_segment(&segments[1])?
//         })
//     }
//
//     fn route_1_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "report" { None? }
//         Some(Self::Report {})
//     }
//
//     fn route_2_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 1 { None? }
//         if segments[0] != "login" { None? }
//         Some(Self::Login {})
//     }
//
//     fn route_3_from_route_segments(segments: &[String]) -> Option<Self> {
//         if segments.len() != 0 { None? }
//         Some(Self::Root {})
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
//                 frequency = routing::encode_uri_component(frequency.into_string_segment()),
//             ).into(),
//             Self::Report => "/report".into(),
//             Self::Login => "/login".into(),
//             Self::Root => "/".into(),
//         }
//     }
// }
// ```

// ------ Route ------

struct Route<'a> {
    ident: &'a Ident,
    fields: Vec<&'a Ident>,
    segments: Vec<RouteSegment>,
}

// ------ RouteSegment ------

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

// ------ route macro ------

#[proc_macro_attribute]
pub fn route(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input_enum: ItemEnum = syn::parse(input)
        .expect("'route' attribute is applicable only to enums and their variants");

    let routes = extract_routes(&mut input_enum);

    let route_fns = generate_route_fns(&routes);
    let impl_from_route_segments = generate_impl_from_route_segments(routes.len());
    let impl_into_cow_str = generate_impl_into_cow_str(&routes);

    quote::quote_spanned!(input_enum.span()=>
        #input_enum
        #route_fns
        #impl_from_route_segments
        #impl_into_cow_str
    )
    .into()
}

// ------ extract_routes ------

fn extract_routes(input_enum: &mut ItemEnum) -> Vec<Route> {
    let mut routes = Vec::new();
    for variant in &mut input_enum.variants {
        let route_attr = take_route_attr(variant);

        let fields: Vec<&Ident> = variant
            .fields
            .iter()
            .map(|field| {
                field
                    .ident
                    .as_ref()
                    .expect("variant can contain only named fields")
            })
            .collect();

        let route = Route {
            ident: &variant.ident,
            fields,
            segments: get_route_segments(&route_attr),
        };
        routes.push(route);
    }
    routes
}

fn take_route_attr(variant: &mut Variant) -> Attribute {
    let route_attr_index = variant
        .attrs
        .iter()
        .position(|attr| {
            attr.path
                .get_ident()
                .map(|ident| ident == "route")
                .unwrap_or_default()
        })
        .expect("'route' attribute is required for all variants");
    variant.attrs.remove(route_attr_index)
}

fn get_route_segments(route_attr: &Attribute) -> Vec<RouteSegment> {
    let parser = Punctuated::<RouteSegment, Token![,]>::parse_terminated;
    route_attr.parse_args_with(parser)
        .expect("only parentheses with zero or more string literals and variant field names are allowed in the 'route' attribute")
        .into_pairs()
        .map(Pair::into_value)
        .collect()
}

// ------ generate_route_fns ------

fn generate_route_fns(routes: &[Route]) -> ItemImpl {
    let route_fns = routes.iter().enumerate().map(route_fn);
    parse_quote!(
        impl Route {
            #(#route_fns)*
        }
    )
}

fn route_fn((index, route): (usize, &Route)) -> ItemFn {
    let fn_name = format_ident!("route_{}_from_route_segments", index);
    let route_segment_count = route.segments.len();
    let lit_str_validations = lit_str_validations(&route.segments);
    let variant_name = route.ident;
    let route_fields = route_fields(&route.segments);
    parse_quote!(
        fn #fn_name(segments: &[String]) -> Option<Self> {
            if segments.len() != #route_segment_count { None? }
            #(#lit_str_validations)*
            Some(Self::#variant_name {
                #(#route_fields),*
            })
        }
    )
}

fn lit_str_validations(segments: &[RouteSegment]) -> impl Iterator<Item = ExprIf> + '_ {
    segments.iter().enumerate().filter_map(|(index, segment)| {
        if let RouteSegment::LitStr(lit_str) = segment {
            Some(parse_quote!(
                if segments[#index] != #lit_str { None? }
            ))
        } else {
            None
        }
    })
}

fn route_fields(segments: &[RouteSegment]) -> impl Iterator<Item = FieldValue> + '_ {
    segments.iter().enumerate().filter_map(|(index, segment)| {
        if let RouteSegment::Ident(ident) = segment {
            Some(parse_quote!(
                #ident: RouteSegment::from_string_segment(&segments[#index])?
            ))
        } else {
            None
        }
    })
}

// ------ generate_impl_from_route_segments ------

fn generate_impl_from_route_segments(route_count: usize) -> ItemImpl {
    let route_fn_idents =
        (0..route_count).map(|index| format_ident!("route_{}_from_route_segments", index));
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

// ------ generate_impl_into_cow_str ------

fn generate_impl_into_cow_str(routes: &[Route]) -> ItemImpl {
    let match_arms = routes.iter().map(match_arm);
    parse_quote!(
        impl<'a> IntoCowStr<'a> for Route {
            fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
                match self {
                    #(#match_arms),*
                }
            }
        }
    )
}

fn match_arm(route: &Route) -> Arm {
    let Route {
        ident,
        fields,
        segments,
    } = route;
    let url_template = assemble_url_template(segments);

    if fields.is_empty() {
        parse_quote!(
            Self::#ident => #url_template.into()
        )
    } else {
        parse_quote!(
            Self::#ident { #(#fields),* } => format!(
                #url_template,
                #(#fields = routing::encode_uri_component(#fields.into_string_segment())),*
            ).into()
        )
    }
}

fn assemble_url_template(segments: &[RouteSegment]) -> LitStr {
    let mut url_template = String::new();
    for segment in segments {
        url_template.push('/');
        match segment {
            RouteSegment::LitStr(lit_str) => url_template.push_str(&url_encode(&lit_str.value())),
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
