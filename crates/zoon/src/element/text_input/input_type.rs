use crate::*;

// ------ InputTypeTrait ------

pub trait InputTypeTrait: Sized {
    const TYPE: &'static str;

    fn apply_to_raw_el<E: RawEl>(self, raw_el: E) -> E {
        raw_el
    }
}

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

impl InputTypeTrait for InputTypeText {
    const TYPE: &'static str = "text";

    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E) -> E {
        if let Some(max_chars) = self.max_chars {
            raw_el = raw_el.attr("maxlength", &max_chars.to_string())
        }
        raw_el
    }
}

// ------ InputTypePassword ------

#[derive(Default)]
pub struct InputTypePassword {
    max_chars:  Option<u32>
}

impl InputTypePassword {
    pub fn max_chars(mut self, max_chars: impl Into<Option<u32>>) -> Self {
        self.max_chars = max_chars.into();
        self
    }
}

impl InputTypeTrait for InputTypePassword {
    const TYPE: &'static str = "password";

    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E) -> E {
        if let Some(max_chars) = self.max_chars {
            raw_el = raw_el.attr("maxlength", &max_chars.to_string())
        }
        raw_el
    }
}

// ------ InputTypeNumber ------

#[derive(Default)]
pub struct InputTypeNumber {
    hide_arrows: bool
}

impl InputTypeNumber {
    pub fn hide_arrows(mut self) -> Self {
        self.hide_arrows = true;
        self
    }
}

impl InputTypeTrait for InputTypeNumber {
    const TYPE: &'static str = "number";

    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E) -> E {
        if self.hide_arrows {
            let webkit_outer_button_group = StyleGroup::new("::-webkit-outer-spin-button")
                .style("margin", "0");

            let webkit_inner_button_group = StyleGroup::new("::-webkit-inner-spin-button")
                .style("margin", "0");

            raw_el = raw_el
                .style_group(webkit_outer_button_group)
                .style_group(webkit_inner_button_group)
                .style("appearance", "textfield")
        }
        raw_el
    }
}

// ------ InputType ------

pub struct InputType;

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
}
