use crate::*;

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
}

impl InputTypePassword {
    pub fn max_chars(mut self, max_chars: impl Into<Option<u32>>) -> Self {
        self.max_chars = max_chars.into();
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

    pub fn dom_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Password(_) => "password",
            Self::Number(_) => "number",
        }
    }

    pub fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E) -> E {
        match self {
            Self::Text(input_type_text) => {
                if let Some(max_chars) = input_type_text.max_chars {
                    raw_el = raw_el.attr("maxlength", &max_chars.to_string())
                }
            }
            Self::Password(input_type_password) => {
                if let Some(max_chars) = input_type_password.max_chars {
                    raw_el = raw_el.attr("maxlength", &max_chars.to_string())
                }
            }
            Self::Number(input_type_number) => {
                if input_type_number.hide_arrows {
                    let webkit_outer_button_group =
                        StyleGroup::new("::-webkit-outer-spin-button").style("margin", "0");

                    let webkit_inner_button_group =
                        StyleGroup::new("::-webkit-inner-spin-button").style("margin", "0");

                    raw_el = raw_el
                        .style_group(webkit_outer_button_group)
                        .style_group(webkit_inner_button_group)
                        .style("appearance", "textfield")
                }
            }
        }
        raw_el
    }
}
