use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(Default)]
pub struct Align {
    alignments: BTreeSet<Alignment>,
    dynamic_alignments: BTreeMap<Alignment, Box<dyn Signal<Item = bool> + Unpin>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum Alignment {
    CenterX,
    CenterY,
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignBottom,
}

impl Align {
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
        self.alignments.insert(Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignLeft);
        self.alignments.remove(&Alignment::AlignRight);
        self
    }

    pub fn center_y(mut self) -> Self {
        self.alignments.insert(Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignTop);
        self.alignments.remove(&Alignment::AlignBottom);
        self
    }

    pub fn top(mut self) -> Self {
        self.alignments.insert(Alignment::AlignTop);
        self.alignments.remove(&Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignBottom);
        self
    }

    pub fn bottom(mut self) -> Self {
        self.alignments.insert(Alignment::AlignBottom);
        self.alignments.remove(&Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignTop);
        self
    }

    pub fn left(mut self) -> Self {
        self.alignments.insert(Alignment::AlignLeft);
        self.alignments.remove(&Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignRight);
        self
    }

    pub fn right(mut self) -> Self {
        self.alignments.insert(Alignment::AlignRight);
        self.alignments.remove(&Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignLeft);
        self
    }
}

impl<'a> Style<'a> for Align {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        for alignment in self.alignments {
            raw_el = raw_el.class(alignment.into());
        }
        for (alignment, enabled) in self.dynamic_alignments {
            raw_el = raw_el.class_signal(<&str>::from(alignment), enabled);
        }
        (raw_el, style_group)
    }
}
