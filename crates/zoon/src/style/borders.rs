use crate::*;
use std::borrow::{Cow, Borrow};

// ------ Borders -----

#[derive(Default)]
pub struct Borders<'a> {
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Borders<'a> {
    pub fn all(self, border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        self.x(border).y(border)
    }

    pub fn x(self, border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        self.left(border).right(border)
    }

    pub fn y(self, border: impl Borrow<Border<'a>>) -> Self {
        let border = border.borrow();
        self.top(border).bottom(border)
    }

    pub fn top(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props.insert("border-top", border.borrow().to_cow_str());
        self
    }

    pub fn right(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props.insert("border-right", border.borrow().to_cow_str());
        self
    }

    pub fn bottom(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props.insert("border-bottom", border.borrow().to_cow_str());
        self
    }

    pub fn left(mut self, border: impl Borrow<Border<'a>>) -> Self {
        self.static_css_props.insert("border-left", border.borrow().to_cow_str());
        self
    }
}

impl<'a> Style<'a> for Borders<'a> {
    fn into_css_props(self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        (self.static_css_props, self.dynamic_css_props)
    }
}

// ------ Border ------

pub struct Border<'a> {
    width: u32,
    style: BorderStyle,
    color: Cow<'a, str>
}

impl<'a> Border<'a> {
    pub fn new() -> Self {
        Self {
            width: 1,
            style: BorderStyle::Solid,
            color: Cow::from("black"),
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: impl Color<'a> + 'a) -> Self {
        self.color = color.into_cow_str();
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
