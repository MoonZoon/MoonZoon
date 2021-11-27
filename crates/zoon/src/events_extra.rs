use crate::*;
use dominator::events::MouseButton;
use web_sys::EventTarget;

// copied from Dominator
macro_rules! make_event {
    ($name:ident, $type:literal => $event:path) => {
        #[derive(Debug)]
        pub struct $name {
            event: $event,
        }

        impl StaticEvent for $name {
            const EVENT_TYPE: &'static str = $type;

            #[inline]
            fn unchecked_from_event(event: web_sys::Event) -> Self {
                Self {
                    event: event.unchecked_into(),
                }
            }
        }

        impl $name {
            #[inline]
            pub fn prevent_default(&self) {
                self.event.prevent_default();
            }

            #[inline]
            pub fn target(&self) -> Option<EventTarget> {
                self.event.target()
            }

            #[inline]
            pub fn dyn_target<A>(&self) -> Option<A>
            where
                A: JsCast,
            {
                self.target()?.dyn_into().ok()
            }
        }
    };
}

// copied from Dominator
macro_rules! make_mouse_event {
    ($name:ident, $type:literal) => {
        make_event!($name, $type => web_sys::MouseEvent);

        impl $name {
            #[inline] pub fn x(&self) -> i32 { self.event.client_x() }
            #[inline] pub fn y(&self) -> i32 { self.event.client_y() }

            #[inline] pub fn screen_x(&self) -> i32 { self.event.screen_x() }
            #[inline] pub fn screen_y(&self) -> i32 { self.event.screen_y() }

            #[inline] pub fn ctrl_key(&self) -> bool { self.event.ctrl_key() || self.event.meta_key() }
            #[inline] pub fn shift_key(&self) -> bool { self.event.shift_key() }
            #[inline] pub fn alt_key(&self) -> bool { self.event.alt_key() }

            // TODO maybe deprecate these ?
            #[inline] pub fn mouse_x(&self) -> i32 { self.event.client_x() }
            #[inline] pub fn mouse_y(&self) -> i32 { self.event.client_y() }

            pub fn button(&self) -> MouseButton {
                match self.event.button() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    3 => MouseButton::Button4,
                    4 => MouseButton::Button5,
                    _ => unreachable!("Unexpected MouseEvent.button value"),
                }
            }
        }
    };
}

make_mouse_event!(MouseOver, "mouseover");
make_mouse_event!(MouseOut, "mouseout");

make_event!(WheelEvent, "wheel" => web_sys::WheelEvent);

// WheelEvent is a subtype of MouseEvent. It implements what MouseEvent implements plus
impl WheelEvent {
    #[inline]
    pub fn x(&self) -> i32 {
        self.event.client_x()
    }
    #[inline]
    pub fn y(&self) -> i32 {
        self.event.client_y()
    }

    #[inline]
    pub fn screen_x(&self) -> i32 {
        self.event.screen_x()
    }
    #[inline]
    pub fn screen_y(&self) -> i32 {
        self.event.screen_y()
    }

    #[inline]
    pub fn ctrl_key(&self) -> bool {
        self.event.ctrl_key() || self.event.meta_key()
    }
    #[inline]
    pub fn shift_key(&self) -> bool {
        self.event.shift_key()
    }
    #[inline]
    pub fn alt_key(&self) -> bool {
        self.event.alt_key()
    }

    // TODO maybe deprecate these ?
    #[inline]
    pub fn mouse_x(&self) -> i32 {
        self.event.client_x()
    }
    #[inline]
    pub fn mouse_y(&self) -> i32 {
        self.event.client_y()
    }

    pub fn button(&self) -> MouseButton {
        match self.event.button() {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Button4,
            4 => MouseButton::Button5,
            _ => unreachable!("Unexpected MouseEvent.button value"),
        }
    }

    #[inline]
    pub fn delta_x(&self) -> f64 {
        self.event.delta_x()
    }
    #[inline]
    pub fn delta_y(&self) -> f64 {
        self.event.delta_y()
    }
    #[inline]
    pub fn delta_z(&self) -> f64 {
        self.event.delta_z()
    }
    #[inline]
    pub fn delta_mode(&self) -> WheelDeltaMode {
        match self.event.delta_mode() {
            web_sys::WheelEvent::DOM_DELTA_PIXEL => WheelDeltaMode::Pixel,
            web_sys::WheelEvent::DOM_DELTA_LINE => WheelDeltaMode::Line,
            web_sys::WheelEvent::DOM_DELTA_PAGE => WheelDeltaMode::Page,
            _ => unreachable!("Unexpected WheelEvent.mode value"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page,
}
