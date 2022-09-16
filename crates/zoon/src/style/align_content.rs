use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(Default)]
pub struct AlignContent {
    alignments: BTreeSet<Alignment>,
    dynamic_alignments: BTreeMap<Alignment, Box<dyn Signal<Item = bool> + Unpin>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum Alignment {
    CenterXContent,
    CenterYContent,
    AlignLeftContent,
    AlignRightContent,
    AlignTopContent,
    AlignBottomContent,
}

impl AlignContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_signal(
        align: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let align = Broadcaster::new(align.map(|align| align.into()));

        for alignment in Alignment::iter() {
            this.dynamic_alignments.insert(
                alignment,
                Box::new(
                    align
                        .signal_ref(move |align| {
                            align
                                .as_ref()
                                .map(|align| align.alignments.contains(&alignment))
                                .unwrap_or_default()
                        })
                        .dedupe(),
                ),
            );
        }
        this
    }

    pub fn center() -> Self {
        Self::default().center_x().center_y()
    }

    pub fn center_x(mut self) -> Self {
        self.alignments.insert(Alignment::CenterXContent);
        self.alignments.remove(&Alignment::AlignLeftContent);
        self.alignments.remove(&Alignment::AlignRightContent);
        self
    }

    pub fn center_y(mut self) -> Self {
        self.alignments.insert(Alignment::CenterYContent);
        self.alignments.remove(&Alignment::AlignTopContent);
        self.alignments.remove(&Alignment::AlignBottomContent);
        self
    }

    pub fn top(mut self) -> Self {
        self.alignments.insert(Alignment::AlignTopContent);
        self.alignments.remove(&Alignment::CenterYContent);
        self.alignments.remove(&Alignment::AlignBottomContent);
        self
    }

    pub fn bottom(mut self) -> Self {
        self.alignments.insert(Alignment::AlignBottomContent);
        self.alignments.remove(&Alignment::CenterYContent);
        self.alignments.remove(&Alignment::AlignTopContent);
        self
    }

    pub fn left(mut self) -> Self {
        self.alignments.insert(Alignment::AlignLeftContent);
        self.alignments.remove(&Alignment::CenterXContent);
        self.alignments.remove(&Alignment::AlignRightContent);
        self
    }

    pub fn right(mut self) -> Self {
        self.alignments.insert(Alignment::AlignRightContent);
        self.alignments.remove(&Alignment::CenterXContent);
        self.alignments.remove(&Alignment::AlignLeftContent);
        self
    }
}

impl<'a> Style<'a> for AlignContent {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            for alignment in self.alignments {
                group = group.class(alignment.into());
            }
            for (alignment, enabled) in self.dynamic_alignments {
                group = group.class_signal(<&str>::from(alignment), enabled);
            }
            group
        });
    }
}
