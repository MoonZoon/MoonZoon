use crate::ui;
use std::{cell::RefCell, rc::Rc};
use zoon::*;

pub struct Dropdown {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
}

impl RawElWrapper for Dropdown 
{
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for Dropdown {}

impl Element for Dropdown {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into_raw_element()
    }
}

impl Dropdown {
    #[track_caller]
    pub fn new<V: IntoCowStr<'static> + Clone + PartialEq + Default + 'static>(
        selected: impl Signal<Item = V> + Unpin + 'static,
        values: impl SignalVec<Item = V> + Unpin + 'static,
        on_select: impl FnMut(V) + 'static,
    ) -> Self {
        let on_select = Rc::new(RefCell::new(on_select));
    
        let on_select_clone = on_select.clone();
        let values_and_selected = map_ref! {
            let values = values.to_signal_cloned(),
            let selected = selected => move {
                let mut values = values.clone();
                if let Some(index) = values.iter().position(|value| value == selected) {
                    values.remove(index);
                    (values, selected.clone())
                } else {
                    let selected = V::default();
                    on_select_clone.borrow_mut()(selected.clone());
                    (values, selected)
                }
            }
        }
        .broadcast();
    
        let values = values_and_selected
            .signal_ref(|(values, _)| values.clone())
            .broadcast();
        let selected = values_and_selected
            .signal_ref(|(_, selected)| selected.clone())
            .broadcast();
    
        let active = Mutable::new(false);
        let hovered = Mutable::new(false);
    
        let raw_el = Column::new()
            .item(
                Row::new()
                    .s(Outline::inner().width(2))
                    .s(Padding::new().x(15).y(10))
                    .s(RoundedCorners::all(3))
                    .s(Cursor::new(CursorIcon::Pointer))
                    .s(Shadows::new([Shadow::new().x(6).y(6)]))
                    .on_click(clone!((active) move || {
                        active.update(not);
                    }))
                    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                    .item(Text::with_signal(selected.signal_cloned()))
                    .item(
                        El::new()
                            .s(Align::new().right())
                            .s(Transform::with_signal(
                                active
                                    .signal()
                                    .map_true(|| Transform::new().flip_vertical()),
                            ))
                            .child(El::new().s(Font::new().weight(FontWeight::Bold)).child("V")),
                    ),
            )
            .on_click_outside(clone!((active) move || active.set_neq(false)))
            .element_below_signal(active.signal().map_true(move || {
                Column::new()
                    .s(Transform::new().move_down(4))
                    .s(Outline::outer().width(2))
                    .s(RoundedCorners::all(3))
                    .s(Scrollbars::both())
                    .s(Shadows::new([Shadow::new().x(15).y(15)]))
                    .items_signal_vec(values.signal_cloned().to_signal_vec().map(
                        clone!((active, on_select) move |value| {
                            Self::menu_item(value, active.clone(), on_select.clone())
                        }),
                    ))
            }))
            .into_raw_el();

        Self {raw_el}
    }
    
    fn menu_item<V: IntoCowStr<'static> + Clone + PartialEq + Default + 'static>(
        value: V,
        dropdown_active: Mutable<bool>,
        on_select: Rc<RefCell<impl FnMut(V) + 'static>>,
    ) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        El::new()
            .s(Cursor::new(CursorIcon::Pointer))
            .s(Padding::new().x(8).y(10))
            .s(Font::new().no_wrap())
            .s(Width::fill())
            .s(Background::new().color_signal(hovered_signal.map_bool(
                || ui::BACKGROUND_COLOR.update_l(|l| l - 15.),
                || ui::BACKGROUND_COLOR,
            )))
            .s(Outline::inner())
            .on_click(clone!((dropdown_active, value, on_select) move || {
                on_select.borrow_mut()(value.clone());
                dropdown_active.set_neq(false);
            }))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .child(value.into_cow_str())
    }
}
