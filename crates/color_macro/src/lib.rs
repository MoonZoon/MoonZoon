use cssparser::{color::PredefinedColorSpace, Parser, ParserInput};
use cssparser_color::Color;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Error, Lit, Token};

// ```
// color!("#fff")
// color!("#FFFFFF", 0.5)
// color!("#FFFFFF", 0)
// color!("Black")
// color!("oklch(0.6 0.182 350.53")
// color!("oklch(0.6 0.182 350.53", 0.5)
// color!("oklch(0.6 0.182 350.53 / .3")
// color!("oklch(0.6 0.182 350.53 / none")
// ```
//
// generates:
//
// ```
// cssparser_color::RgbaLegacy { red: 255, green: 255, blue: 255, alpha: 1.0 }
// cssparser_color::RgbaLegacy { red: 255, green: 255, blue: 255, alpha: 0.5 }
// cssparser_color::RgbaLegacy { red: 255, green: 255, blue: 255, alpha: 0.0 }
// cssparser_color::RgbaLegacy { red: 0, green: 0, blue: 0, alpha: 1.0 }
// cssparser_color::Oklch { lightness: Some(0.6), chroma: Some(0.182), hue: Some(350.53), alpha: Some(1.0) }
// cssparser_color::Oklch { lightness: Some(0.6), chroma: Some(0.182), hue: Some(350.53), alpha: Some(0.5) }
// cssparser_color::Oklch { lightness: Some(0.6), chroma: Some(0.182), hue: Some(350.53), alpha: Some(0.3) }
// cssparser_color::Oklch { lightness: Some(0.6), chroma: Some(0.182), hue: Some(350.53), alpha: None }
// ```

#[proc_macro]
pub fn color(input: TokenStream) -> TokenStream {
    let args_parser = Punctuated::<Lit, Token![,]>::parse_terminated;
    let args = parse_macro_input!(input with args_parser);

    if args.len() != 1 && args.len() != 2 {
        return Error::new_spanned(args, "1 or 2 arguments expected")
            .into_compile_error()
            .into();
    };

    let color: String = match &args[0] {
        Lit::Str(color_lit) => color_lit.value(),
        lit => {
            return Error::new_spanned(lit, "string literal expected")
                .into_compile_error()
                .into()
        }
    };

    let extra_alpha: Option<f32> = match (args.len() == 2).then(|| &args[1]) {
        Some(Lit::Float(lit_float)) => match lit_float.to_string().parse() {
            Ok(alpha) => Some(alpha),
            Err(error) => {
                return Error::new_spanned(lit_float, error)
                    .into_compile_error()
                    .into()
            }
        },
        Some(Lit::Int(lit_int)) => match lit_int.to_string().parse() {
            Ok(alpha) => Some(alpha),
            Err(error) => {
                return Error::new_spanned(lit_int, error)
                    .into_compile_error()
                    .into()
            }
        },
        Some(lit) => {
            return Error::new_spanned(lit, "float or int literal expected")
                .into_compile_error()
                .into()
        }
        None => None,
    };

    if let Some(alpha) = extra_alpha {
        if alpha < 0.0 {
            return Error::new_spanned(&args[1], "alpha >= 0.0 expected")
                .into_compile_error()
                .into();
        } else if alpha > 1.0 {
            return Error::new_spanned(&args[1], "alpha <= 1.0 expected")
                .into_compile_error()
                .into();
        }
    }

    let color = match Color::parse(&mut Parser::new(&mut ParserInput::new(&color))) {
        Ok(color) => color,
        Err(error) => {
            return Error::new_spanned(
                &args[0],
                format!("failed to parse CSS color '{color}': {error:?}"),
            )
            .into_compile_error()
            .into()
        }
    };

    fn quote_option<T: ToTokens>(value: Option<T>) -> TokenStream2 {
        if let Some(value) = value {
            quote!(Some(#value))
        } else {
            quote!(None)
        }
    }

    let color = match color {
        Color::ColorFunction(cssparser_color::ColorFunction {
            color_space,
            c1,
            c2,
            c3,
            alpha,
        }) => {
            let color_space = match color_space {
                PredefinedColorSpace::A98Rgb => {
                    quote!(cssparser::color::PredefinedColorSpace::A98Rgb)
                }
                PredefinedColorSpace::DisplayP3 => {
                    quote!(cssparser::color::PredefinedColorSpace::DisplayP3)
                }
                PredefinedColorSpace::ProphotoRgb => {
                    quote!(cssparser::color::PredefinedColorSpace::ProphotoRgb)
                }
                PredefinedColorSpace::Rec2020 => {
                    quote!(cssparser::color::PredefinedColorSpace::Rec2020)
                }
                PredefinedColorSpace::Srgb => quote!(cssparser::color::PredefinedColorSpace::Srgb),
                PredefinedColorSpace::SrgbLinear => {
                    quote!(cssparser::color::PredefinedColorSpace::SrgbLinear)
                }
                PredefinedColorSpace::XyzD50 => {
                    quote!(cssparser::color::PredefinedColorSpace::XyzD50)
                }
                PredefinedColorSpace::XyzD65 => {
                    quote!(cssparser::color::PredefinedColorSpace::XyzD65)
                }
            };
            let c1 = quote_option(c1);
            let c2 = quote_option(c2);
            let c3 = quote_option(c3);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::ColorFunction { color_space: #color_space, c1: #c1, c2: #c2, c3: #c3, alpha: #alpha })
        }
        Color::CurrentColor => {
            quote!(cssparser_color::Color::CurrentColor)
        }
        Color::Hsl(cssparser_color::Hsl {
            hue,
            saturation,
            lightness,
            alpha,
        }) => {
            let hue = quote_option(hue);
            let saturation = quote_option(saturation);
            let lightness = quote_option(lightness);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Hsl { hue: #hue, saturation: #saturation, lightness: #lightness, alpha: #alpha })
        }
        Color::Hwb(cssparser_color::Hwb {
            hue,
            whiteness,
            blackness,
            alpha,
        }) => {
            let hue = quote_option(hue);
            let whiteness = quote_option(whiteness);
            let blackness = quote_option(blackness);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Hwb { hue: #hue, whiteness: #whiteness, blackness: #blackness, alpha: #alpha })
        }
        Color::Lab(cssparser_color::Lab {
            lightness,
            a,
            b,
            alpha,
        }) => {
            let lightness = quote_option(lightness);
            let a = quote_option(a);
            let b = quote_option(b);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Lab { lightness: #lightness, a: #a, b: #b, alpha: #alpha })
        }
        Color::Lch(cssparser_color::Lch {
            lightness,
            chroma,
            hue,
            alpha,
        }) => {
            let lightness = quote_option(lightness);
            let chroma = quote_option(chroma);
            let hue = quote_option(hue);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Lch { lightness: #lightness, chroma: #chroma, hue: #hue, alpha: #alpha })
        }
        Color::Oklab(cssparser_color::Oklab {
            lightness,
            a,
            b,
            alpha,
        }) => {
            let lightness = quote_option(lightness);
            let a = quote_option(a);
            let b = quote_option(b);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Oklab { lightness: #lightness, a: #a, b: #b, alpha: #alpha })
        }
        Color::Oklch(cssparser_color::Oklch {
            lightness,
            chroma,
            hue,
            alpha,
        }) => {
            let lightness = quote_option(lightness);
            let chroma = quote_option(chroma);
            let hue = quote_option(hue);
            let alpha = quote_option(extra_alpha.or(alpha));
            quote!(cssparser_color::Oklch { lightness: #lightness, chroma: #chroma, hue: #hue, alpha: #alpha })
        }
        Color::Rgba(cssparser_color::RgbaLegacy {
            red,
            green,
            blue,
            alpha,
        }) => {
            let alpha = extra_alpha.unwrap_or(alpha);
            quote!(cssparser_color::RgbaLegacy { red: #red, green: #green, blue: #blue, alpha: #alpha })
        }
    };
    quote_spanned!(args.span()=> #color).into()
}
