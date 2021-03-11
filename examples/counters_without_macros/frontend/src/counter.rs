#![no_std]

use zoon::*;

#[macro_export]
macro_rules! counter {
    ( $($item:expr),* $(,)?) => {
        {
            let mut counter = counter::Counter::default();
            $( counter = counter.set($item); )*
            counter
        }
    }
}

#[derive(Default)]
pub struct Counter {
    value: usize,
    on_change: Option<OnChange>
}

impl Counter {
    pub fn set(mut self, item: impl ApplyToCounter) -> Self {
        item.apply_to_counter(&mut self);
        self
    }
}

impl Component for Counter {
    #[render]
    fn render(&mut self, cx: Cx) {
        let value = el_var(|| 0_usize);
        column![
            button![button::on_press(|| value.update(|value| value - 1)), "-"],
            value.inner(),
            button![button::on_press(|| value.update(|value| value + 1)), "+"],
        ].render(cx)
    }
}

pub trait ApplyToCounter {
    fn apply_to_counter(self, counter: &mut Counter);
}

impl ApplyToCounter for usize {
    fn apply_to_counter(self, counter: &mut Counter) {
        counter.value = self;
    }
}

pub struct OnChange(Box<dyn Fn(usize)>);
pub fn on_change(on_change: impl FnOnce(usize) + Clone + 'static) -> OnChange {
    OnChange(Box::new(move |value| on_change.clone()(value)))
}
impl ApplyToCounter for OnChange {
    fn apply_to_counter(self, counter: &mut Counter) {
        counter.on_change = Some(self);
    }
}
