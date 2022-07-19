use crate::*;
use std::borrow::Cow;

// ------ Cursor ------
/// Styling to manage mouse cursor.
#[derive(Default)]
pub struct Cursor<'a> {
    /// Default static properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable properties that can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    /// let help_button = Button::new().s(Cursor::new(CursorIcon::Help));
    /// ```
    pub fn new(cursor_icon: impl Into<Option<CursorIcon<'a>>>) -> Self {
        let mut this = Self::default();
        if let Some(cursor_icon) = cursor_icon.into() {
            this.static_css_props
                .insert("cursor", cursor_icon.into_cow_str());
        }
        this
    }

    /// Create a new cursor depending of the signal's state.
    /// # Example
    /// This example updates the cursor when the user clicks the button.
    /// ```no_run
    /// use zoon::*;
    /// let (pressing, pressing_signal) = Mutable::new_and_signal(false);
    /// let help_button = Button::new()
    ///     .s(Cursor::with_signal(pressing_signal.map_true(|| CursorIcon::Grabbing)))
    ///     .on_pointer_down(clone!((pressing) move || pressing.set_neq(true)))
    ///     .on_pointer_up(clone!((pressing) move || pressing.set_neq(false)))
    ///     .on_pointer_leave(move || pressing.set_neq(false))
    ///     .label("Click me");
    /// ```
    pub fn with_signal(
        cursor_icon: impl Signal<Item = impl Into<Option<CursorIcon<'static>>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let cursor_icon = cursor_icon.map(|cursor_icon| {
            cursor_icon
                .into()
                .map(|cursor_icon| cursor_icon.into_cow_str())
        });
        this.dynamic_css_props
            .insert("cursor".into(), box_css_signal(cursor_icon));
        this
    }
}

impl<'a> Style<'a> for Cursor<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self {
                static_css_props,
                dynamic_css_props,
            } = self;
            group.static_css_props.extend(static_css_props);
            group.dynamic_css_props.extend(dynamic_css_props);
            group
        });
    }
}

// ------ CursorIcon ------

#[derive(Debug, Clone)]
pub enum CursorIcon<'a> {
    // -- General --
    Auto,
    Default,
    None,
    // -- Links & Status --
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    // -- Selection --
    Cell,
    Crosshair,
    Text,
    VerticalText,
    // -- Drag & Drop --
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    // -- Resizing & Panning --
    Pan,
    ColumnResize,
    RowResize,
    // -- Arrows --
    UpArrow,
    RightArrow,
    LeftArrow,
    DownArrow,
    UpRightArrow,
    UpLeftArrow,
    DownRightArrow,
    DownLeftArrow,
    LeftRightArrow,
    UpDownArrow,
    UpRightDownLeftArrow,
    UpLeftDownRightArrow,
    Custom(Cow<'a, str>),
}

impl<'a> CursorIcon<'a> {
    pub fn new(url: &str, hotspot: impl Into<Option<(u32, u32)>>) -> Self {
        let url = url.into_cow_str();
        let hotspot = match hotspot.into() {
            Some((x, y)) => crate::format!(" {} {}", x, y),
            None => String::new(),
        };
        let cursor_value = crate::format!("url({}){}, auto", url, hotspot);
        CursorIcon::Custom(cursor_value.into())
    }
}

impl<'a> IntoCowStr<'a> for CursorIcon<'a> {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            // -- General --
            CursorIcon::Auto => "auto".into(),
            CursorIcon::Default => "default".into(),
            CursorIcon::None => "none".into(),
            // -- Links & Status --
            CursorIcon::ContextMenu => "context-menu".into(),
            CursorIcon::Help => "help".into(),
            CursorIcon::Pointer => "pointer".into(),
            CursorIcon::Progress => "progress".into(),
            CursorIcon::Wait => "wait".into(),
            // -- Selection --
            CursorIcon::Cell => "cell".into(),
            CursorIcon::Crosshair => "crosshair".into(),
            CursorIcon::Text => "text".into(),
            CursorIcon::VerticalText => "vertical-text".into(),
            // -- Drag & Drop --
            CursorIcon::Alias => "alias".into(),
            CursorIcon::Copy => "copy".into(),
            CursorIcon::Move => "move".into(),
            CursorIcon::NoDrop => "no-drop".into(),
            CursorIcon::NotAllowed => "not-allowed".into(),
            CursorIcon::Grab => "grab".into(),
            CursorIcon::Grabbing => "grabbing".into(),
            // -- Resizing & Panning --
            CursorIcon::Pan => "all-scroll".into(),
            CursorIcon::ColumnResize => "col-resize".into(),
            CursorIcon::RowResize => "row-resize".into(),
            // -- Arrows --
            CursorIcon::UpArrow => "n-resize".into(),
            CursorIcon::RightArrow => "e-resize".into(),
            CursorIcon::LeftArrow => "s-resize".into(),
            CursorIcon::DownArrow => "w-resize".into(),
            CursorIcon::UpRightArrow => "ne-resize".into(),
            CursorIcon::UpLeftArrow => "nw-resize".into(),
            CursorIcon::DownRightArrow => "se-resize".into(),
            CursorIcon::DownLeftArrow => "sw-resize".into(),
            CursorIcon::LeftRightArrow => "ew-resize".into(),
            CursorIcon::UpDownArrow => "ns-resize".into(),
            CursorIcon::UpRightDownLeftArrow => "nesw-resize".into(),
            CursorIcon::UpLeftDownRightArrow => "nwse-resize".into(),
            CursorIcon::Custom(cursor_value) => cursor_value.into(),
        }
    }
}
