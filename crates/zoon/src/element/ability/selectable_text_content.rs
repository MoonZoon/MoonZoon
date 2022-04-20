use crate::*;

// ------ SelectableTextContent ------

pub trait SelectableTextContent: UpdateRawEl + Sized {
    fn text_content_selecting(self, selecting: TextContentSelecting) -> Self {
        self.update_raw_el(|raw_el| raw_el.style("user-select", selecting.user_select))
    }
}

// ------ TextContentSelecting ------

/// See https://developer.mozilla.org/en-US/docs/Web/CSS/user-select (including Browser compatibility)
#[derive(Clone, Copy)]
pub struct TextContentSelecting {
    user_select: &'static str,
}

impl Default for TextContentSelecting {
    fn default() -> Self {
        Self {
            user_select: "auto",
        }
    }
}

impl TextContentSelecting {
    pub fn none() -> Self {
        Self {
            user_select: "none",
        }
    }

    pub fn all() -> Self {
        Self { user_select: "all" }
    }

    // @TODO uncomment of remove?
    // pub fn text() -> Self {
    //     Self { user_select: "text" }
    // }

    pub fn contain() -> Self {
        Self {
            user_select: "contain",
        }
    }
}
