use zoon::*;
use std::rc::Rc;
use enclose::enc;

// ------ ------
//    Element 
// ------ ------

#[derive(Default)]
pub struct Counter {
    value: Option<i32>,
    on_change: Option<Rc<dyn Fn(i32)>>,
    step: Option<i32>,
}

impl Element for Counter {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        let on_change = self.on_change.take().map(|on_change| on_change);
        let step = self.step.unwrap_or(1);
        
        let value = el_var(|| 0);
        if let Some(required_value) = self.value {
            value.set(required_value);
        }
        
        let update_value = move |delta: i32| {
            value.update(|value| value + delta);
            if let Some(on_change) = on_change.clone() {
                on_change(value.inner());
            }
            rcx.rerender();
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

impl Counter {
    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }

    pub fn on_change(mut self, on_change: impl FnOnce(i32) + Clone + 'static) -> Self {
        self.on_change = Some(Rc::new(move |value| on_change.clone()(value)));
        self
    }

    pub fn step(mut self, step: i32) -> Self {
        self.step = Some(step);
        self
    }
}
