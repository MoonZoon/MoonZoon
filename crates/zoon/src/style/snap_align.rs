use crate::*;

#[derive(Default, Clone)]
pub struct SnapAlign<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> SnapAlign<'a> {
    pub fn start() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("scroll-snap-align", "start");
        this
    }

    pub fn center() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("scroll-snap-align", "center");
        this
    }

    pub fn end() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("scroll-snap-align", "end");
        this
    }
}

impl<'a> Style<'a> for SnapAlign<'a> {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            let Self { static_css_props } = self;
            group.static_css_props.extend(static_css_props);
            group
        });
    }
}
