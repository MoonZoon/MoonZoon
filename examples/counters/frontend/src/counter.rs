use zoon::*;

// TODO uncomment no_std and resolve Boxes
// TODO macro which generates macro below (`component_macro!(counter, Counter);`)

#[macro_export]
macro_rules! counter {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut counter = $crate::counter::Counter::default();
            $( counter = counter.set($attribute); )*
            counter
        }
    }
}

// ------ ------
//   Component 
// ------ ------

component_macro!(counter, Counter);

#[derive(Default)]
pub struct Counter {
    value: usize,
    on_change: Option<OnChange>,
}

impl Component for Counter {
    // #[render]
    fn render(&mut self, render_context: RenderContext) {
        // let value = el_var(|| 0_usize);
        // column![
        //     button![button::on_press(|| value.update(|value| value - 1)), "-"],
        //     value.inner(),
        //     button![button::on_press(|| value.update(|value| value + 1)), "+"],
        // ].render(render_context);
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
