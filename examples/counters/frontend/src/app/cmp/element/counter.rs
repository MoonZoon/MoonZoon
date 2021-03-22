use zoon::*;
use std::rc::Rc;
use enclose::enc;

// ------ ------
//    Element 
// ------ ------

element_macro!(counter, Counter::default());

#[derive(Default)]
pub struct Counter {
    value: Option<i32>,
    on_change: Option<OnChange>,
    step: Option<i32>,
}

impl Element for Counter {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("Counter render: {:#?}", TrackedCallId::current());

        let on_change = self.on_change.take().map(|on_change| on_change.0);
        let step = self.step.unwrap_or(1);
        
        let value = l_var(|| 0);
        if let Some(required_value) = self.value {
            value.set(required_value);
        }
        
        let update_value = move |delta: i32| {
            value.update(|value| value + delta);
            if let Some(on_change) = on_change.clone() {
                on_change(value.inner());
            }
        };
        row![
            button![
                button::on_press(enc!((update_value) move || update_value(-step))),
                "-"
            ],
            el![value.inner().to_string()],
            button![
                button::on_press(move || update_value(step)), 
                 "+"
            ],
        ].render(rcx);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ i32 ------

impl ApplyToElement<Counter> for i32 {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.value = Some(self);
    }
}

// ------ counter::on_change(...) -------

pub struct OnChange(Rc<dyn Fn(i32)>);

pub fn on_change(on_change: impl FnOnce(i32) + Clone + 'static) -> OnChange {
    OnChange(Rc::new(move |value| on_change.clone()(value)))
}

impl ApplyToElement<Counter> for OnChange {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.on_change = Some(self);
    }
}

// ------ counter::step(...) -------

pub struct Step(i32);

pub fn step(step: i32) -> Step {
    Step(step)
}

impl ApplyToElement<Counter> for Step {
    fn apply_to_element(self, counter: &mut Counter) {
        counter.step = Some(self.0);
    }
}
