use crate::{web_sys::HtmlDivElement, *};
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Label);

pub struct Checkbox<IdFlag, OnChangeFlag, LabelFlag> {
    checked: Mutable<bool>,
    raw_el: RawHtmlEl,
    flags: PhantomData<(IdFlag, OnChangeFlag, LabelFlag)>,
}

impl
    Checkbox<
        IdFlagNotSet,
        OnChangeFlagNotSet,
        LabelFlagNotSet,
    >
{
    pub fn new() -> Self {
        let checked = Mutable::new(false);
        Self {
            checked: checked.clone(),
            raw_el: RawHtmlEl::new("div")
                .attr("class", "checkbox")
                .attr("role", "checkbox")
                .attr("aria-live", "polite")
                .attr("tabindex", "0")
                .attr_signal("aria-checked", checked.signal().map(|checked| checked.to_string()))
                .style("cursor", "pointer")
                .event_handler(move |_: events::Click| checked.update(|checked| !checked)),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag> Element
    for Checkbox<IdFlagSet, OnChangeFlag, LabelFlagNotSet>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag> Element
    for Checkbox<IdFlagNotSet, OnChangeFlag, LabelFlagSet>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, LabelFlag> UpdateRawEl<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, LabelFlag> Styleable<'_, RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag> KeyboardEventAware<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag> Focusable
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag> MouseEventAware<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, LabelFlag> Hookable<RawHtmlEl>
    for Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
    type WSElement = HtmlDivElement;
}
impl<IdFlag, OnChangeFlag, LabelFlag> AddNearby<'_> for Checkbox<IdFlag, OnChangeFlag, LabelFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, IdFlag, OnChangeFlag, LabelFlag>
    Checkbox<IdFlag, OnChangeFlag, LabelFlag>
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> Checkbox<IdFlagSet, OnChangeFlag, LabelFlag>
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn checked(
        self,
        checked: bool,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag>
    // where
    //     TextFlag: FlagNotSet,
    {
        self.checked.set_neq(checked);
        self.into_type()
    }

    pub fn checked_signal(
        mut self,
        checked: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag>
    // where
    //     TextFlag: FlagNotSet,
    {
        let checked_mutable = self.checked.clone();
        let checked_changer = checked.for_each(move |checked| {
            checked_mutable.set_neq(checked);
            async {}
        });
        let task_handle = Task::start_droppable(checked_changer);
        self.raw_el = self.raw_el.after_remove(move |_| drop(task_handle));
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnOnce(bool) + Clone + 'static,
    ) -> Checkbox<IdFlag, OnChangeFlagSet, LabelFlag>
    where
        OnChangeFlag: FlagNotSet,
    {
        let on_change = move |checked| on_change.clone()(checked);
        let on_change_invoker = self.checked.signal().for_each(move |checked| { 
            on_change(checked);
            async {}
        });
        let task_handle = Task::start_droppable(on_change_invoker);
        self.raw_el = self.raw_el.after_remove(move |_| drop(task_handle));
        self.into_type()
    }

    pub fn label_hidden(
        mut self,
        label: impl IntoCowStr<'a>,
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlagSet>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("aria-label", &label.into_cow_str());
        self.into_type()
    }

    pub fn icon<IE: IntoElement<'a> + 'a>(
        mut self,
        icon: impl FnOnce(MutableSignal<bool>) -> IE
    ) -> Checkbox<IdFlag, OnChangeFlag, LabelFlag> {
        let icon = icon(self.checked.signal());
        self.raw_el = self.raw_el.child(icon);
        self.into_type()
    }

    fn into_type<NewIdFlag, NewOnChangeFlag, NewLabelFlag>(
        self,
    ) -> Checkbox<NewIdFlag, NewOnChangeFlag, NewLabelFlag> {
        Checkbox {
            checked: self.checked,
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
 
    El::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Background::new().url_signal(checked_signal.map_bool(
            || COMPLETED_ICON, || ACTIVE_ICON
        )))
}
