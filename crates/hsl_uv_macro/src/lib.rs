use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, Token, Lit, spanned::Spanned, Error};
use quote::quote_spanned;

// ```
// hsl!(89.5, 100, 90.)
// hsl!(265, 85.9, 32, 80.2)
// hsl!(265, 85.9, 32, 20)
// ```
//
// generates:
//
// ```
// HSLuv::new_unchecked(89.5, 100., 90., 100. }
// HSLuv::new_unchecked(265., 85.9, 32., 80.2 }
// HSLuv::new_unchecked(265., 85.9, 32., 20. }
// ```

#[proc_macro]
pub fn hsl(input: TokenStream) -> TokenStream {
    let args_parser = Punctuated::<Lit, Token![,]>::parse_terminated;
    let args = parse_macro_input!(input with args_parser);

    let mut numbers = Vec::new();
    for arg in &args {
        match arg {
            Lit::Float(lit_float) => {
                match lit_float.to_string().parse() {
                    Ok(number) => numbers.push(number),
                    Err(error) => return Error::new_spanned(lit_float, error).into_compile_error().into(),
                }
            }
            Lit::Int(lit_int) => {
                match lit_int.to_string().parse() {
                    Ok(number) => numbers.push(number),
                    Err(error) => return Error::new_spanned(lit_int, error).into_compile_error().into(),
                }
            }
            lit => return Error::new_spanned(lit, "only float and int literals expected").into_compile_error().into()
        }
    }

    let [h, s, l, a] = match numbers.as_slice() {
        [h, s, l] => [h, s, l, &100.],
        [h, s, l, a] => [h, s, l, a],
        _ => return Error::new_spanned(args, "3 (hsl) or 4 (hsla) arguments expected").into_compile_error().into(),
    };

    if *h < 0. || *h > 360. {
        return Error::new_spanned(&args[0], "h (hue) value in the range 0..=360 expected").into_compile_error().into()
    }
    if *s < 0. || *s > 100. {
        return Error::new_spanned(&args[1], "s (saturation) value in the range 0..=100 expected").into_compile_error().into()
    }
    if *l < 0. || *l > 100. {
        return Error::new_spanned(&args[2], "l (lightness) value in the range 0..=100 expected").into_compile_error().into()
    }
    if *a < 0. || *a > 100. {
        return Error::new_spanned(&args[3], "a (alpha channel) in the range 0..=100 expected").into_compile_error().into()
    }

    quote_spanned!(args.span() => HSLuv::new_unchecked(#h, #s, #l, #a)).into()
}
