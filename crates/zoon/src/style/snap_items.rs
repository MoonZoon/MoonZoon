use crate::*;

pub struct SnapItems<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Default for SnapItems<'a> {
    fn default() -> Self {
        Self {
            static_css_props: StaticCSSProps::default(),
        }
    }
}

impl<'a> SnapItems<'a> {
    pub fn both() -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("scroll-snap-type", "both mandatory");
        this
    }

    pub fn x() -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("scroll-snap-type", "x mandatory");
        this
    }

    pub fn y() -> Self {
        let mut this = Self::default();
        this.static_css_props
            .insert("scroll-snap-type", "y mandatory");
        this
    }
}

impl<'a> Style<'a> for SnapItems<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self { static_css_props } = self;
            group.static_css_props.extend(static_css_props);
            group
        });
    }
}
