use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// Styling to align elements inside their container.
/// It is possible to combine different methods to get the styling you need.
///
/// Here the element is positioned on the bottom of the right side.
/// # Example
/// ```no_run
/// use zoon::*;
///
/// let right_bottom_element = El::new()
///     .s(Font::new().size(50).weight(FontWeight::Bold))
///     .s(Align::new().bottom().right())
///     .child("bottom_right element");
/// ```
///
/// Here the element is vertically aligned on the right side.
/// # Example
/// ```no_run
/// use zoon::*;
///
/// let right_bottom_element = El::new()
///     .s(Font::new().size(50).weight(FontWeight::Bold))
///     .s(Align::new().right().center_y())
///     .child("bottom_right element");
/// ```
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the alignment depending on Signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (is_hovered, hover_signal) = Mutable::new_and_signal(false);
    ///
    /// let container = Column::new().s(Height::screen()).item(
    ///     Button::new()
    ///         .s(Align::with_signal(
    ///             hover_signal.map_bool(|| Align::default().bottom(), Align::center),
    ///         ))
    ///         .on_hovered_change(move |hover| is_hovered.set(hover))
    ///         .label("hover me"),
    /// );
    /// ```
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
    /// Align an element in the center of its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let centered_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::center())
    ///     .child("centered element");
    /// ```
    pub fn center() -> Self {
        Self::default().center_x().center_y()
    }

    /// The element will be centered horizontally.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let horizontal_centered_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().center_x())
    ///     .child("Horizontally centered element");
    ///  ```
    pub fn center_x(mut self) -> Self {
        self.alignments.insert(Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignLeft);
        self.alignments.remove(&Alignment::AlignRight);
        self
    }

    /// The element will be centered vertically.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let vertically_centered_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().center_y())
    ///     .child("Vertically centered header");
    ///  ```
    pub fn center_y(mut self) -> Self {
        self.alignments.insert(Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignTop);
        self.alignments.remove(&Alignment::AlignBottom);
        self
    }

    /// The element will be aligned at the top of its container.
    /// By default the element is also positioned on left side as well.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let top_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().top())
    ///     .child("Top element");
    ///  ```
    pub fn top(mut self) -> Self {
        self.alignments.insert(Alignment::AlignTop);
        self.alignments.remove(&Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignBottom);
        self
    }

    /// The element will be aligned at the bottom of its container.
    /// By default the element is also positioned on left side as well.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let bottom_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().bottom())
    ///     .child("Bottom element");
    ///  ```
    pub fn bottom(mut self) -> Self {
        self.alignments.insert(Alignment::AlignBottom);
        self.alignments.remove(&Alignment::CenterY);
        self.alignments.remove(&Alignment::AlignTop);
        self
    }

    /// The element will be aligned at the left of its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let bottom_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().left())
    ///     .child("Left element");
    ///  ```
    pub fn left(mut self) -> Self {
        self.alignments.insert(Alignment::AlignLeft);
        self.alignments.remove(&Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignRight);
        self
    }

    /// The element will be aligned at the right of its container.
    /// By default the element is also positioned on top as well.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let right_element = El::new()
    ///     .s(Font::new().size(50).weight(FontWeight::Bold))
    ///     .s(Align::new().right())
    ///     .child("Right element");
    ///  ```
    pub fn right(mut self) -> Self {
        self.alignments.insert(Alignment::AlignRight);
        self.alignments.remove(&Alignment::CenterX);
        self.alignments.remove(&Alignment::AlignLeft);
        self
    }
}

impl<'a> Style<'a> for Align {
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
