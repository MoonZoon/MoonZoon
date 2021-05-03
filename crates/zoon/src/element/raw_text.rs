use crate::{Element, RawElement, IntoElement, IntoOptionElement};
use dominator::{Dom, DomBuilder, traits::StaticEvent};
use futures_signals::{signal::{Signal, SignalExt}, signal_vec::{SignalVec, SignalVecExt}};

// ------ ------
//   Element 
// ------ ------

pub struct RawText {
    dom: Dom,
}

impl RawText {
    pub fn with_text(text: impl AsRef<str>) -> Self {
        Self {
            dom: dominator::text(text.as_ref()),
        }
    }

    pub fn with_signal(text: impl Signal<Item = impl ToString> + Unpin + 'static) -> Self {
        Self {
            dom: dominator::text_signal(text.map(|text| text.to_string()))
        }
    }
}

impl Element for RawText {
    fn render(self) -> Dom {
        self.dom
    }
}

impl RawElement for RawText {
    fn render(self) -> Dom {
        self.dom
    }
}
