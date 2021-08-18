use crate::{web_sys::HtmlDivElement, *};
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckState {
    NotSet,
    Value(bool),
    FirstValue(bool),
}

make_flags!(Id, OnChange, Label, Icon, Checked);

pub struct Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> {
    check_state: Mutable<CheckState>,
    raw_el: RawHtmlEl,
    flags: PhantomData<(IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag)>,
}

impl
    Checkbox<IdFlagNotSet, OnChangeFlagNotSet, LabelFlagNotSet, IconFlagNotSet, CheckedFlagNotSet>
{
    pub fn new() -> Self {
        let check_state = Mutable::new(CheckState::NotSet);
        Self {
            check_state: check_state.clone(),
            raw_el: RawHtmlEl::new("div")
                .class("checkbox")
                .attr("role", "checkbox")
                .attr("aria-live", "polite")
                .attr("tabindex", "0")
                .attr_signal(
                    "aria-checked",
                    check_state.signal().map(|check_state| match check_state {
                        CheckState::NotSet => None,
                        CheckState::FirstValue(checked) | CheckState::Value(checked) => {
                            Some(checked.to_string())
                        }
                    }),
                )
                .style("cursor", "pointer")
                .event_handler(move |_: events::Click| {
                    check_state.update(|check_state| match check_state {
                        CheckState::NotSet => CheckState::FirstValue(true),
                        CheckState::FirstValue(checked) | CheckState::Value(checked) => {
                            CheckState::Value(!checked)
                        }
                    })
                }),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, CheckedFlag> Element
    for Checkbox<IdFlagSet, OnChangeFlag, LabelFlagNotSet, IconFlagSet, CheckedFlag>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, CheckedFlag> Element
    for Checkbox<IdFlagNotSet, OnChangeFlag, LabelFlagSet, IconFlagSet, CheckedFlag>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> UpdateRawEl<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> Styleable<'_, RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> KeyboardEventAware<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> Focusable
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> MouseEventAware<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> Hookable<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
    type WSElement = HtmlDivElement;
}
impl<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag> AddNearbyElement<'_>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
    Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> Checkbox<IdFlagSet, OnChangeFlag, LabelFlag, IconFlag, CheckedFlag>
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn checked(
        self,
        checked: bool,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlagSet>
    where
        CheckedFlag: FlagNotSet,
    {
        self.check_state.update(|check_state| match check_state {
            CheckState::NotSet => CheckState::FirstValue(checked),
            CheckState::FirstValue(_) | CheckState::Value(_) => CheckState::Value(checked),
        });
        self.into_type()
    }

    pub fn checked_signal(
        mut self,
        checked: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlag, CheckedFlagSet>
    where
        CheckedFlag: FlagNotSet,
    {
        let check_state = self.check_state.clone();
        let checked_changer = checked.for_each(move |checked| {
            let new_state = match check_state.get() {
                CheckState::NotSet => CheckState::FirstValue(checked),
                CheckState::FirstValue(_) | CheckState::Value(_) => CheckState::Value(checked),
            };
            check_state.set_neq(new_state);
            async {}
        });
        let task_handle = Task::start_droppable(checked_changer);
        self.raw_el = self.raw_el.after_remove(move |_| drop(task_handle));
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnOnce(bool) + Clone + 'static,
    ) -> Checkbox<IdFlag, OnChangeFlagSet, LabelFlag, IconFlag, CheckedFlag>
    where
        OnChangeFlag: FlagNotSet,
    {
        let on_change = move |checked| on_change.clone()(checked);
        let on_change_invoker = self.check_state.signal().for_each(move |check_state| {
            if let CheckState::Value(checked) = check_state {
                on_change(checked);
            }
            async {}
        });
        let task_handle = Task::start_droppable(on_change_invoker);
        self.raw_el = self.raw_el.after_remove(move |_| drop(task_handle));
        self.into_type()
    }

    pub fn label_hidden(
        mut self,
        label: impl IntoCowStr<'a>,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlagSet, IconFlag, CheckedFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("aria-label", &label.into_cow_str());
        self.into_type()
    }

    pub fn icon<IE: IntoElement<'a> + 'a>(
        mut self,
        icon: impl FnOnce(Mutable<bool>) -> IE,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag, IconFlagSet, CheckedFlag>
    where
        IconFlag: FlagNotSet,
    {
        fn is_checked(check_state: CheckState) -> bool {
            match check_state {
                CheckState::NotSet => false,
                CheckState::FirstValue(checked) | CheckState::Value(checked) => checked,
            }
        }

        let checked = Mutable::new(is_checked(self.check_state.get()));
        let icon = icon(checked.clone());

        let check_state = self.check_state.clone();
        let task_handle =
            Task::start_droppable(check_state.signal().for_each(move |check_state| {
                checked.set_neq(is_checked(check_state));
                async {}
            }));

        self.raw_el = self.raw_el.child(icon).after_remove(|_| drop(task_handle));
        self.into_type()
    }

    fn into_type<NewIdFlag, NewOnChangeFlag, NewLabelFlag, NewIconFlag, NewCheckedFlag>(
        self,
    ) -> Checkbox<NewIdFlag, NewOnChangeFlag, NewLabelFlag, NewIconFlag, NewCheckedFlag> {
        Checkbox {
            check_state: self.check_state,
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

// ------ ------
//     Extra
// ------ ------

pub fn default_icon(checked_signal: MutableSignal<bool>) -> impl Element {
    // @TODO replace with better custom icons
    // Icons from https://github.com/tastejs/todomvc
    static ACTIVE_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E";
    static COMPLETED_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E";

    El::new().s(Width::new(40)).s(Height::new(40)).s(
        Background::new().url_signal(checked_signal.map_bool(|| COMPLETED_ICON, || ACTIVE_ICON))
    )
}
