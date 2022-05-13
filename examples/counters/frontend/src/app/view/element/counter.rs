use std::marker::PhantomData;
use std::{iter, rc::Rc};
use zoon::*;

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

fn decrement_button(on_press: impl FnMut() + 'static) -> impl Element {
    Button::new().label("-").on_press(on_press)
}

fn value_element<'a>(
    value: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    El::new().child(Text::with_signal(value))
}

fn increment_button(on_press: impl FnMut() + 'static) -> impl Element {
    Button::new().label("+").on_press(on_press)
}

fn counter(
    decrement_button: impl Element,
    value_element: impl Element,
    increment_button: impl Element,
) -> RawElement {
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
            decrement_button(clone!((on_change) move || on_change(-step))),
            value_element(self.value_signal.unwrap_throw()),
            increment_button(move || on_change(step)),
        )
    }
}

impl<ValueFlag, StepFlag> Element
    for Counter<ValueFlag, ValueSignalFlagNotSet, OnChangeFlagNotSet, StepFlag>
{
    fn into_raw_element(self) -> RawElement {
        let state_value = Rc::new(Mutable::new(self.value));
        let step = self.step;
        counter(
            decrement_button(clone!((state_value) move || {
                state_value.update(|value| value - step)
            })),
            value_element(state_value.signal()),
            increment_button(move || state_value.update(|value| value + step)),
        )
    }
}

impl<ValueFlag, StepFlag> Element
    for Counter<ValueFlag, ValueSignalFlagNotSet, OnChangeFlagSet, StepFlag>
{
    fn into_raw_element(self) -> RawElement {
        let state_value = Rc::new(Mutable::new(self.value));
        let on_change = self.on_change.unwrap_throw();
        let step = self.step;
        counter(
            decrement_button(clone!((state_value, on_change) move || {
                state_value.update(|value| value - step);
                on_change(-step);
            })),
            value_element(state_value.signal()),
            increment_button(move || {
                state_value.update(|value| value + step);
                on_change(step);
            }),
        )
    }
}

impl<ValueFlag, ValueSignal, OnChangeFlag, StepFlag> IntoIterator
    for Counter<ValueFlag, ValueSignal, OnChangeFlag, StepFlag>
{
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl<ValueFlag, ValueSignalFlag, OnChangeFlag, StepFlag>
    Counter<ValueFlag, ValueSignalFlag, OnChangeFlag, StepFlag>
{
    pub fn value(
        mut self,
        value: i32,
    ) -> Counter<ValueFlagSet, ValueSignalFlagNotSet, OnChangeFlag, StepFlag>
    where
        ValueFlag: FlagNotSet,
        ValueSignalFlag: FlagNotSet,
    {
        self.value = value;
        self.into_type()
    }

    pub fn value_signal(
        mut self,
        value: impl Signal<Item = i32> + Unpin + 'static,
    ) -> Counter<ValueFlagNotSet, ValueSignalFlagSet, OnChangeFlag, StepFlag>
    where
        ValueFlag: FlagNotSet,
        ValueSignalFlag: FlagNotSet,
    {
        self.value_signal = Some(Box::new(value));
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnMut(i32) + 'static,
    ) -> Counter<ValueFlag, ValueSignalFlag, OnChangeFlagSet, StepFlag>
    where
        OnChangeFlag: FlagNotSet,
    {
        self.on_change = Some(Rc::new(move |value| on_change.clone()(value)));
        self.into_type()
    }

    pub fn step(
        mut self,
        step: i32,
    ) -> Counter<ValueFlag, ValueSignalFlag, OnChangeFlag, StepFlagSet>
    where
        StepFlag: FlagNotSet,
    {
        self.step = step;
        self.into_type()
    }

    fn into_type<NewValueFlag, NewValueSignalFlag, NewOnChangeFlag, NewStepFlag>(
        self,
    ) -> Counter<NewValueFlag, NewValueSignalFlag, NewOnChangeFlag, NewStepFlag> {
        Counter {
            value: self.value,
            value_signal: self.value_signal,
            on_change: self.on_change,
            step: self.step,
            flags: PhantomData,
        }
    }
}
