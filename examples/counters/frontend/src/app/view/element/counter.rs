use zoon::*;
use std::rc::Rc;
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

make_flags!(Value, ValueSignal, OnChange, Step);

pub type CounterStep = i32; 

pub struct Counter<ValueFlag, ValueSignal, OnChangeFlag, StepFlag> {
    value: i32,
    value_signal: Option<Box<dyn Signal<Item = i32> + Unpin>>,
    on_change: Option<Rc<dyn Fn(CounterStep)>>,
    step: CounterStep,
    flags: PhantomData<(ValueFlag, ValueSignal, OnChangeFlag, StepFlag)>,
}

impl Counter<ValueFlagNotSet, ValueSignalFlagNotSet, OnChangeFlagNotSet, StepFlagNotSet> {
    pub fn new() -> Self {
        Self {
            value: 0,
            value_signal: None,
            on_change: None,
            step: 1,
            flags: PhantomData,
        }
    }
}

fn decrement_button() -> Button<button::LabelFlagSet, button::OnPressFlagNotSet> {
    Button::new().label("-")
}

fn value_element(value: impl Signal<Item = impl ToString> + Unpin + 'static) -> impl Element {
    El::new().child(Text::with_signal(value))
}

fn increment_button() -> Button<button::LabelFlagSet, button::OnPressFlagNotSet> {
    Button::new().label("+")
}

fn counter(decrement_button: impl Element, value_element: impl Element, increment_button: impl Element) -> RawElement {
    Row::new()
        .item(decrement_button)
        .item(value_element)
        .item(increment_button)
        .into_raw_element()
}

impl<StepFlag> Element for Counter<ValueFlagNotSet, ValueSignalFlagSet, OnChangeFlagSet, StepFlag> {
    fn into_raw_element(self) -> RawElement {
        let on_change = self.on_change.unwrap_throw();
        let step = self.step;
        counter(
            decrement_button().on_press(clone!((on_change) move || on_change(-step))),
            value_element(self.value_signal.unwrap_throw()),
            increment_button().on_press(move || on_change(step))
        )
    }
}

impl<ValueFlag, StepFlag> Element for Counter<ValueFlag, ValueSignalFlagNotSet, OnChangeFlagNotSet, StepFlag> {
    fn into_raw_element(self) -> RawElement {
        let state_value = Rc::new(Mutable::new(self.value));
        let step = self.step;
        counter(
            decrement_button().on_press(clone!((state_value) move || {
                state_value.update(|value| value - step)
            })),
            value_element(state_value.signal()),
            increment_button().on_press(move || {
                state_value.update(|value| value + step)
            }),
        )
    }
}

impl<ValueFlag, StepFlag> Element for Counter<ValueFlag, ValueSignalFlagNotSet, OnChangeFlagSet, StepFlag> {
    fn into_raw_element(self) -> RawElement {
        let state_value = Rc::new(Mutable::new(self.value));
        let on_change = self.on_change.unwrap_throw();
        let step = self.step;
        counter(
            decrement_button().on_press(clone!((state_value, on_change) move || {
                state_value.update(|value| value - step);
                on_change(-step);
            })),
            value_element(state_value.signal()),
            increment_button().on_press(move || {
                state_value.update(|value| value + step);
                on_change(step);
            })
        )
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<ValueFlag, ValueSignal, OnChangeFlag, StepFlag> Counter<ValueFlag, ValueSignal, OnChangeFlag, StepFlag> {
    pub fn value(
        self, 
        value: i32
    ) -> Counter<ValueFlagSet, ValueSignalFlagNotSet, OnChangeFlag, StepFlag>
        where 
            ValueFlag: FlagNotSet,
            ValueSignal: FlagNotSet,
    {
        Counter {
            value,
            value_signal: None,
            on_change: self.on_change,
            step: self.step,
            flags: PhantomData,
        }
    }

    pub fn value_signal(
        self, 
        value: impl Signal<Item  = i32> + Unpin + 'static
    ) -> Counter<ValueFlagNotSet, ValueSignalFlagSet, OnChangeFlag, StepFlag>
        where 
            ValueFlag: FlagNotSet,
            ValueSignal: FlagNotSet,
    {
        Counter {
            value: 0,
            value_signal: Some(Box::new(value)),
            on_change: self.on_change,
            step: self.step,
            flags: PhantomData,
        }
    }

    pub fn on_change(
        self, 
        on_change: impl FnOnce(i32) + Clone + 'static
    ) -> Counter<ValueFlag, ValueSignal, OnChangeFlagSet, StepFlag>
        where OnChangeFlag: FlagNotSet
    {
        Counter {
            value: self.value,
            value_signal: self.value_signal,
            on_change: Some(Rc::new(move |value| on_change.clone()(value))),
            step: self.step,
            flags: PhantomData,
        }
    }

    pub fn step(
        self, 
        step: i32
    ) -> Counter<ValueFlag, ValueSignal, OnChangeFlag, StepFlagSet>
        where StepFlag: FlagNotSet
    {
        Counter {
            value: self.value,
            value_signal: self.value_signal,
            on_change: self.on_change,
            step,
            flags: PhantomData,
        }
    }
}
