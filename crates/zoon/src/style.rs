use std::collections::BTreeMap;

pub trait Style<'a> {
    fn into_css_props(self) -> BTreeMap<&'a str, &'a str>;
}

// ------ Font ------

pub struct Font<'a> {
    css_props: BTreeMap<&'a str, &'a str>
}

impl<'a> Font<'a> {
    pub fn new() -> Self {
        Self {
            css_props: BTreeMap::new()
        }
    }

    pub fn bold(self) -> Self {
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
    fn into_css_props(self) -> BTreeMap<&'a str, &'a str> {
        self.css_props
    }
}
