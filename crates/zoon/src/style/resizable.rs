use crate::*;

#[derive(Default, Clone)]
pub struct Resizable<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Resizable<'a> {
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("resize", "both");
        this
    }

    pub fn x() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("resize", "horizontal");
        this
    }

    pub fn y() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("resize", "vertical");
        this
    }

    pub fn none() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("resize", "none");
        this
    }
}

impl<'a> Style<'a> for Resizable<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self { static_css_props } = self;
            group.static_css_props.extend(static_css_props);
            group
        });
    }
}
