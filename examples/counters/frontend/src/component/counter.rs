use zoon::*;

// ------ ------
//   Component 
// ------ ------

// component_macro!(counter, Counter);

#[macro_export]
macro_rules! counter {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut counter = $crate::component::counter::Counter::default();
            $( counter = counter.with($attribute); )*
            counter
        }
    }
}

#[derive(Default)]
pub struct Counter {
    value: usize,
    on_change: Option<OnChange>,
}

impl Component for Counter {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        let value = el_var(|| 0_usize);
        col![
            button![button::on_press(move || value.update(|value| value - 1)), text!("-")],
            text!(value.inner().to_string()),
            button![button::on_press(move || value.update(|value| value + 1)), text!("+")],
        ].render(rcx);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ usize ------

impl ApplyToComponent<Counter> for usize {
    fn apply_to_component(self, component: &mut Counter) {
        component.value = self;
    }
}

// ------ counter::on_change(...) -------

pub struct OnChange(Box<dyn Fn(usize)>);

pub fn on_change(on_change: impl FnOnce(usize) + Clone + 'static) -> OnChange {
    OnChange(Box::new(move |value| on_change.clone()(value)))
}

impl ApplyToComponent<Counter> for OnChange {
    fn apply_to_component(self, counter: &mut Counter) {
        counter.on_change = Some(self);
    }
}
