use crate::*;
use std::pin::Pin;

// ------ InputTypeText ------

#[derive(Default)]
pub struct InputTypeText {
    max_chars: Option<u32>,
}

impl InputTypeText {
    pub fn max_chars(mut self, max_chars: impl Into<Option<u32>>) -> Self {
        self.max_chars = max_chars.into();
        self
    }
}

impl From<InputTypeText> for InputType {
    fn from(input_type_text: InputTypeText) -> Self {
        InputType::Text(input_type_text)
    }
}

// ------ InputTypePassword ------

#[derive(Default)]
pub struct InputTypePassword {
    max_chars: Option<u32>,
    mask_signal: Option<Pin<Box<dyn Signal<Item = Option<bool>>>>>,
}

impl InputTypePassword {
    pub fn max_chars(mut self, max_chars: impl Into<Option<u32>>) -> Self {
        self.max_chars = max_chars.into();
        self
    }

    pub fn mask_signal(
        mut self,
        mask: impl Signal<Item = impl Into<Option<bool>>> + 'static,
    ) -> Self {
        self.mask_signal = Some(mask.map(|mask| mask.into()).boxed_local());
        self
    }
}

impl From<InputTypePassword> for InputType {
    fn from(input_type_password: InputTypePassword) -> Self {
        InputType::Password(input_type_password)
    }
}

// ------ InputTypeNumber ------

#[derive(Default)]
pub struct InputTypeNumber {
    hide_arrows: bool,
}

impl InputTypeNumber {
    pub fn hide_arrows(mut self) -> Self {
        self.hide_arrows = true;
        self
    }
}

impl From<InputTypeNumber> for InputType {
    fn from(input_type_number: InputTypeNumber) -> Self {
        InputType::Number(input_type_number)
    }
}

// ------ InputType ------

pub enum InputType {
    Text(InputTypeText),
    Password(InputTypePassword),
    Number(InputTypeNumber),
}

impl InputType {
    pub fn text() -> InputTypeText {
        InputTypeText::default()
    }

    pub fn password() -> InputTypePassword {
        InputTypePassword::default()
    }

    pub fn number() -> InputTypeNumber {
        InputTypeNumber::default()
    }

    pub fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E) -> E {
        match self {
            Self::Text(InputTypeText { max_chars }) => {
                if let Some(max_chars) = max_chars {
                    raw_el = raw_el.attr("maxlength", &max_chars.to_string())
                }
                raw_el
            }
            Self::Password(InputTypePassword {
                max_chars,
                mask_signal,
            }) => {
                if let Some(max_chars) = max_chars {
                    raw_el = raw_el.attr("maxlength", &max_chars.to_string())
                }
                if let Some(mask_signal) = mask_signal {
                    let mask_signal = mask_signal
                        .map(|mask| mask.map(|mask| if mask { "password" } else { "text" }));
                    // @TODO replace with `input-security` once possible
                    // https://github.com/Fyrd/caniuse/issues/2297
                    return raw_el.attr_signal("type", mask_signal);
                }
                raw_el.attr("type", "password")
            }
            Self::Number(InputTypeNumber { hide_arrows }) => {
                if hide_arrows {
                    let webkit_outer_button_group =
                        StyleGroup::new("::-webkit-outer-spin-button").style("margin", "0");

                    let webkit_inner_button_group =
                        StyleGroup::new("::-webkit-inner-spin-button").style("margin", "0");

                    raw_el = raw_el
                        .style_group(webkit_outer_button_group)
                        .style_group(webkit_inner_button_group)
                        .style("appearance", "textfield")
                }
                raw_el.attr("type", "number")
            }
        }
    }
}
