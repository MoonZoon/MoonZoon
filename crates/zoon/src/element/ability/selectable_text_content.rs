use crate::*;

// ------ SelectableTextContent ------

pub trait SelectableTextContent: RawElWrapper + Sized {
    fn text_content_selecting(self, selecting: impl Into<Option<TextContentSelecting>>) -> Self {
        if let Some(selecting) = selecting.into() {
            return self.update_raw_el(|raw_el| raw_el.style("user-select", selecting.user_select));
        }
        self
    }

    fn text_content_selecting_signal(
        self,
        selecting: impl Signal<Item = impl Into<Option<TextContentSelecting>>> + Unpin + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.style_signal(
                "user-select",
                selecting.map(|selecting| selecting.into().map(|selecting| selecting.user_select)),
            )
        })
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
        Self::auto()
    }
}

impl TextContentSelecting {
    pub fn auto() -> Self {
        Self {
            user_select: "auto",
        }
    }

    pub fn none() -> Self {
        Self {
            user_select: "none",
        }
    }

    pub fn all() -> Self {
        Self { user_select: "all" }
    }

    pub fn text() -> Self {
        Self {
            user_select: "text",
        }
    }

    pub fn contain() -> Self {
        Self {
            user_select: "contain",
        }
    }
}
