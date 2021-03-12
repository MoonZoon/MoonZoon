use zoon::*;
use std::rc::Rc;
use enclose::enc;

// ------ ------
//   Component 
// ------ ------

component_macro!(counter, Counter::default());

#[derive(Default)]
pub struct Counter {
    value: i32,
    on_change: Option<OnChange>,
}

impl Component for Counter {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        let on_change = self.on_change.take().map(|on_change| on_change.0);
        let value = el_var(|| 0);
        
        let update_value = move |delta: i32| {
            value.update(|value| value + delta);
            if let Some(on_change) = on_change.clone() {
                on_change(value.inner());
            }
        };
        col![
            button![
                button::on_press(enc!((update_value) move || update_value(-1))),
                "-"
            ],
            text!(value.inner().to_string()),
            button![
                button::on_press(move || update_value(1)), 
                 "+"
            ],
        ].render(rcx);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ i32 ------

impl ApplyToComponent<Counter> for i32 {
    fn apply_to_component(self, counter: &mut Counter) {
        counter.value = self;
    }
}

// ------ counter::on_change(...) -------

pub struct OnChange(Rc<dyn Fn(i32)>);

pub fn on_change(on_change: impl FnOnce(i32) + Clone + 'static) -> OnChange {
    OnChange(Rc::new(move |value| on_change.clone()(value)))
}

impl ApplyToComponent<Counter> for OnChange {
    fn apply_to_component(self, counter: &mut Counter) {
        counter.on_change = Some(self);
    }
}
