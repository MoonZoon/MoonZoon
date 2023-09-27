use zoon::{
    num_traits::{Num, SaturatingAdd, SaturatingSub},
    *,
};

pub struct Counter<N> {
    raw_el: RawHtmlEl,
    step: Mutable<N>,
    value: Mutable<N>,
}

impl<N> Element for Counter<N> {}
impl<N> Styleable<'_> for Counter<N> {}

impl<N> RawElWrapper for Counter<N> {
    type RawEl = RawHtmlEl;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl<N> Counter<N>
where
    N: Num + SaturatingAdd + SaturatingSub + Copy + IntoCowStr<'static> + 'static,
{
    pub fn new(default_value: N) -> Self {
        let step = Mutable::new(N::one());
        let value = Mutable::new(default_value);
        Self {
            raw_el: Row::new()
                .item(
                    Button::new()
                        .label("-")
                        .on_press(clone!((step, value) move || {
                            value.update(|value| value.saturating_sub(&step.get()))
                        })),
                )
                .item(El::new().child(Text::with_signal(value.signal())))
                .item(
                    Button::new()
                        .label("+")
                        .on_press(clone!((step, value) move || {
                            value.update(|value| value.saturating_add(&step.get()))
                        })),
                )
                .into_raw_el(),
            step,
            value,
        }
    }

    pub fn with_signal(value_signal: impl Signal<Item = N> + 'static) -> Self {
        let this = Self::new(N::zero());
        let value_updater = Task::start_droppable(value_signal.for_each_sync(
            clone!((this.value => value) move |new_value| {
                value.set_neq(new_value);
            }),
        ));
        this.update_raw_el(|raw_el| raw_el.after_remove(move |_| drop(value_updater)))
    }

    pub fn on_change(self, on_change: impl FnMut(N) + 'static) -> Self {
        let on_change_invoker = Task::start_droppable(self.value.signal().for_each_sync(on_change));
        self.update_raw_el(|raw_el| raw_el.after_remove(move |_| drop(on_change_invoker)))
    }

    pub fn step(self, step: N) -> Self {
        self.step.set(step);
        self
    }
}
