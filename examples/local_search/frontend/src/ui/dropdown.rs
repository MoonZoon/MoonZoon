use crate::BACKGROUND_COLOR;
use std::{cell::RefCell, rc::Rc};
use zoon::*;

pub struct Dropdown {
    raw_el: RawHtmlEl,
}

impl Element for Dropdown {}

impl RawElWrapper for Dropdown {
    type RawEl = RawHtmlEl;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for Dropdown {}

impl Dropdown {
    pub fn new<V, IE>(
        selected_value: impl Signal<Item = V> + Unpin + 'static,
        values: impl SignalVec<Item = V> + Unpin + 'static,
        value_to_label: impl FnMut(&V) -> IE + 'static,
        on_select: impl FnMut(&V) + 'static,
    ) -> Self
    where
        V: Clone + PartialEq + 'static,
        IE: IntoElement<'static>,
    {
        let selected_value = selected_value.broadcast();
        let (filtered_values, filtered_values_updater) =
            filter_values(values, selected_value.clone());
        let dropdown_active = Mutable::new(false);

        let value_to_label = {
            let value_to_label = Rc::new(RefCell::new(value_to_label));
            move |value: &V| value_to_label.borrow_mut()(value).into_element().into_raw()
        };

        let raw_el = Column::new()
            .after_remove(move |_| drop(filtered_values_updater))
            .item(head(
                dropdown_active.clone(),
                selected_value.signal_ref(value_to_label.clone()),
            ))
            .on_click_outside(clone!((dropdown_active) move || dropdown_active.set_neq(false)))
            .element_below_signal({
                let on_select = {
                    let on_select = Rc::new(RefCell::new(on_select));
                    move |value: &V| on_select.borrow_mut()(value)
                };
                dropdown_active.signal().map_true(move || {
                    menu(
                        filtered_values.clone(),
                        dropdown_active.clone(),
                        value_to_label.clone(),
                        on_select.clone(),
                    )
                })
            })
            .into_raw_el();

        Self { raw_el }
    }
}

fn filter_values<V>(
    values: impl SignalVec<Item = V> + Unpin + 'static,
    selected_value: Broadcaster<impl Signal<Item = V> + Unpin + 'static>,
) -> (MutableVec<V>, TaskHandle)
where
    V: Clone + PartialEq + 'static,
{
    let filtered_values = MutableVec::new();
    let filtered_values_updater = Task::start_droppable(values
        .filter_signal_cloned(clone!((selected_value) move |value| {
            selected_value.signal_ref(clone!((value) move |selected_value| selected_value != &value))
        }))
        // @TODO implement `for_each_sync` and remove `async {}` below
        .for_each(clone!((filtered_values) move |vec_diff| {
            MutableVecLockMut::apply_vec_diff(&mut filtered_values.lock_mut(), vec_diff);
            async {}
        }))
    );
    (filtered_values, filtered_values_updater)
}

fn head(
    dropdown_active: Mutable<bool>,
    selected_value_label: impl Signal<Item = RawElOrText> + Unpin + 'static,
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Row::new()
        .s(Outline::inner().width(2))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all(3))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().x(6).y(6)]))
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || BACKGROUND_COLOR.also(|color| *color.lightness.get_or_insert(1.0) += 0.1),
            || BACKGROUND_COLOR,
        )))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(clone!((dropdown_active) move || {
            dropdown_active.update(not);
        }))
        .item_signal(selected_value_label)
        .item(
            El::new()
                .s(Align::new().right())
                .s(Transform::with_signal(
                    dropdown_active
                        .signal()
                        .map_true(|| Transform::new().flip_vertical()),
                ))
                .child(El::new().s(Font::new().weight(FontWeight::Bold)).child("V")),
        )
}

fn menu<V>(
    filtered_values: MutableVec<V>,
    dropdown_active: Mutable<bool>,
    value_to_label: impl Fn(&V) -> RawElOrText + Clone + 'static,
    on_select: impl Fn(&V) + Clone + 'static,
) -> impl Element
where
    V: Clone + PartialEq + 'static,
{
    Column::new()
        .s(Transform::new().move_down(4))
        .s(Outline::outer().width(2))
        .s(RoundedCorners::all(3))
        .s(Scrollbars::both())
        .s(Shadows::new([Shadow::new().x(15).y(15)]))
        .items_signal_vec(filtered_values.signal_vec_cloned().map(move |value| {
            menu_item(
                value.clone(),
                dropdown_active.clone(),
                value_to_label.clone(),
                on_select.clone(),
            )
        }))
}

fn menu_item<V>(
    value: V,
    dropdown_active: Mutable<bool>,
    value_to_label: impl Fn(&V) -> RawElOrText + Clone + 'static,
    on_select: impl Fn(&V) + Clone + 'static,
) -> impl Element
where
    V: Clone + PartialEq + 'static,
{
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Padding::new().x(8).y(10))
        .s(Font::new().no_wrap())
        .s(Width::fill())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || BACKGROUND_COLOR.also(|color| *color.lightness.get_or_insert(1.0) += 0.1),
            || BACKGROUND_COLOR,
        )))
        .s(Outline::inner())
        .on_click(clone!((value) move || {
            on_select(&value);
            dropdown_active.set_neq(false);
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .child(value_to_label(&value))
}
