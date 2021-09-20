use crate::*;
use std::{
    borrow::{Borrow, Cow},
    sync::Arc,
};

// ------ Borders ------

#[derive(Default)]
pub struct Borders<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
    task_handles: Vec<TaskHandle>,
}

impl<'a> Borders<'a> {
    pub fn all(border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        Self::default().x(border).y(border)
    }

    pub fn all_signal(border: impl Signal<Item = Border<'static>> + Unpin + 'static) -> Self {
        let mutable = Mutable::new(Border::new());
        let mut this = Self::default()
            .x_signal(mutable.signal_cloned())
            .y_signal(mutable.signal_cloned());
        this.task_handles
            .push(Task::start_droppable(border.for_each_sync(
                move |new_border| {
                    mutable.set(new_border);
                },
            )));
        this
    }

    pub fn x(self, border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        self.left(border).right(border)
    }

    pub fn x_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let mutable = Mutable::new(Border::new());
        self = self
            .left_signal(mutable.signal_cloned())
            .right_signal(mutable.signal_cloned());
        self.task_handles
            .push(Task::start_droppable(border.for_each_sync(
                move |new_border| {
                    mutable.set(new_border);
                },
            )));
        self
    }

    pub fn y(self, border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        self.top(border).bottom(border)
    }

    pub fn y_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let mutable = Mutable::new(Border::new());
        self = self
            .top_signal(mutable.signal_cloned())
            .bottom_signal(mutable.signal_cloned());
        self.task_handles
            .push(Task::start_droppable(border.for_each_sync(
                move |new_border| {
                    mutable.set(new_border);
                },
            )));
        self
    }

    pub fn top(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props
            .insert("border-top", border.borrow().to_cow_str());
        self
    }

    pub fn top_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-top".into(), box_css_signal(border));
        self
    }

    pub fn bottom(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props
            .insert("border-bottom", border.borrow().to_cow_str());
        self
    }

    pub fn bottom_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-bottom".into(), box_css_signal(border));
        self
    }

    pub fn right(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props
            .insert("border-right", border.borrow().to_cow_str());
        self
    }

    pub fn right_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-right".into(), box_css_signal(border));
        self
    }

    pub fn left(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props
            .insert("border-left", border.borrow().to_cow_str());
        self
    }

    pub fn left_signal(
        mut self,
        border: impl Signal<Item = Border<'static>> + Unpin + 'static,
    ) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-left".into(), box_css_signal(border));
        self
    }
}

impl<'a> Style<'a> for Borders<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        let task_handles = self.task_handles;
        if not(task_handles.is_empty()) {
            raw_el = raw_el.after_remove(move |_| drop(task_handles))
        }
        if let Some(mut style_group) = style_group {
            for (name, value) in self.static_css_props {
                style_group = style_group.style(name, value);
            }
            for (name, value) in self.dynamic_css_props {
                style_group = style_group.style_signal(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}

// ------ Border ------

#[derive(Clone)]
pub struct Border<'a> {
    width: u32,
    style: BorderStyle,
    color: Arc<Cow<'a, str>>,
}

impl<'a> Border<'a> {
    pub fn new() -> Self {
        Self {
            width: 1,
            style: BorderStyle::Solid,
            color: Arc::new(Cow::from("black")),
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: impl Color<'a> + 'a) -> Self {
        self.color = Arc::new(color.into_cow_str());
        self
    }

    pub fn solid(mut self) -> Self {
        self.style = BorderStyle::Solid;
        self
    }

    pub fn dashed(mut self) -> Self {
        self.style = BorderStyle::Dashed;
        self
    }

    pub fn dotted(mut self) -> Self {
        self.style = BorderStyle::Dotted;
        self
    }

    fn to_cow_str(&self) -> Cow<'a, str> {
        crate::format!("{}px {} {}", self.width, self.style.as_str(), &self.color).into()
    }
}

// ------ BorderStyle ------

#[derive(Clone, Copy)]
enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}

impl BorderStyle {
    fn as_str(&self) -> &str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}
