use super::{Filter, Todo};
use crate::router::{router, Route};
use std::sync::Arc;
use strum::IntoEnumIterator;
use zoon::*;

pub fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::fill().min_screen())
        .s(Font::new()
            .size(14)
            .color(hsl(0, 0, 5.1))
            .weight(NamedWeight::Light)
            .family(vec![
                FontFamily::new("Helvetica Neue"),
                FontFamily::new("Helvetica"),
                FontFamily::new("Arial"),
                FontFamily::SansSerif,
            ]))
        .s(Background::new().color(hsl(0, 0, 96.5)))
        .item(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::fill().min(230).max(550))
        .s(Align::new().center_x())
        .item(header())
        .item(
            Column::new()
                .s(Width::fill())
                .s(Spacing::new(65))
                .item(panel())
                .item(footer()),
        )
}

fn header() -> impl Element {
    El::with_tag(Tag::Header)
        .s(Padding::new().top(10))
        .s(Align::new().center_x())
        .s(Height::new(130))
        .s(Font::new()
            .size(100)
            .color(hsla(10.5, 62.8, 44.5, 15))
            .weight(NamedWeight::Hairline))
        .child(El::with_tag(Tag::H1).child("todos"))
}

fn panel() -> impl Element {
    Column::with_tag(Tag::Section)
        .s(Shadows::new(vec![
            Shadow::new().y(2).blur(4).color(hsla(0, 0, 0, 20)),
            Shadow::new().y(25).blur(50).color(hsla(0, 0, 0, 10)),
        ]))
        .s(Width::fill())
        .s(Background::new().color(hsl(0, 0, 100)))
        .item(new_todo_title())
        .item_signal(super::todos_exist().map_true(todos))
        .item_signal(super::todos_exist().map_true(panel_footer))
}

fn new_todo_title() -> impl Element {
    TextInput::new()
        .s(Padding::new().y(19).left(60).right(16))
        .s(Font::new().size(24).color(hsl(0, 0, 32.7)))
        .s(Background::new().color(hsla(0, 0, 0, 0.3)))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(-2)
            .blur(1)
            .color(hsla(0, 0, 0, 3))]))
        .focused()
        .on_change(super::set_new_todo_title)
        .label_hidden("What needs to be done?")
        .placeholder(
            Placeholder::new("What needs to be done?")
                .s(Font::new().italic().color(hsl(0, 0, 91.3))),
        )
        .on_key_down(|event| event.if_key(Key::Enter, super::add_todo))
        .text_signal(super::new_todo_title().signal_cloned())
}

fn todos() -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(hsl(0, 0, 91.3))))
        .s(Background::new().color(hsl(0, 0, 93.7)))
        .s(Spacing::new(1))
        .items_signal_vec(super::filtered_todos().map(todo))
        .element_above(toggle_all_checkbox())
}

fn toggle_all_checkbox() -> impl Element {
    Checkbox::new()
        .s(Width::new(60))
        .s(Height::fill())
        .checked_signal(super::are_all_completed().signal())
        .on_click(super::check_or_uncheck_all_todos)
        .label_hidden("Toggle all")
        .icon(|checked| {
            El::new()
                .s(Font::new().size(22).color_signal(
                    checked
                        .signal()
                        .map_bool(|| hsl(0, 0, 48.4), || hsl(0, 0, 91.3)),
                ))
                .s(Transform::new().rotate(90).move_up(18))
                .s(Height::new(34))
                .s(Padding::new().x(27).y(6))
                .child("❯")
        })
}

fn todo(todo: Arc<Todo>) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(hsl(0, 0, 100)))
        .s(Spacing::new(5))
        .s(Font::new().size(24))
        .items_signal_vec(
            super::is_todo_selected(todo.id)
                .map(move |selected| todo_content(todo.clone(), selected))
                .to_signal_vec(),
        )
}

fn todo_content(todo: Arc<Todo>, selected: bool) -> Vec<impl Element> {
    if selected {
        return element_vec![editing_todo_title(todo)];
    }
    element_vec![todo_checkbox(todo.clone()), todo_title(todo),]
}

fn todo_checkbox(todo: Arc<Todo>) -> impl Element {
    static ACTIVE_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E";
    static COMPLETED_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E";

    Checkbox::new()
        .id(todo.id.to_string())
        .checked_signal(todo.completed.signal())
        .on_change(move |checked| super::set_todo_completed(&todo, checked))
        .icon(|checked| {
            El::new()
                .s(Width::new(40))
                .s(Height::new(40))
                .s(Background::new()
                    .url_signal(checked.signal().map_bool(|| COMPLETED_ICON, || ACTIVE_ICON)))
        })
}

fn todo_title(todo: Arc<Todo>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Label::new()
        .s(Width::fill())
        .s(Font::new()
            .color_signal(
                todo.completed
                    .signal()
                    .map_bool(|| hsl(0, 0, 86.7), || hsl(0, 0, 32.7)),
            )
            .strike_signal(todo.completed.signal())
            .size(24))
        .s(Padding::all(15).right(60))
        .s(Clip::x())
        .for_input(todo.id.to_string())
        .label_signal(todo.title.signal_cloned())
        .on_double_click(clone!((todo )move || super::select_todo(Some(todo))))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .element_on_right_signal(hovered_signal.map_true(move || remove_todo_button(&todo)))
}

fn remove_todo_button(todo: &Todo) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = todo.id;
    Button::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Transform::new().move_left(50).move_down(14))
        .s(Font::new().size(30).center().color_signal(
            hovered_signal.map_bool(|| hsl(10.5, 37.7, 48.8), || hsl(12.2, 34.7, 68.2)),
        ))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || super::remove_todo(id))
        .label("×")
}

fn editing_todo_title(todo: Arc<Todo>) -> impl Element {
    let text_signal = todo.edited_title.signal_cloned().map(Option::unwrap_throw);
    TextInput::new()
        .s(Width::new(506))
        .s(Padding::all(17).bottom(16))
        .s(Align::new().right())
        .s(Borders::all(Border::new().color(hsl(0, 0, 63.2))))
        .s(Shadows::new(vec![Shadow::new()
            .inner()
            .y(-1)
            .blur(5)
            .color(hsla(0, 0, 0, 20))]))
        .s(Font::new().color(hsl(0, 0, 32.7)))
        .label_hidden("selected todo title")
        .focused()
        .on_blur(super::save_selected_todo)
        .on_change(move |text| todo.edited_title.set_neq(Some(text)))
        .on_key_down(|event| match event.key() {
            Key::Escape => super::select_todo(None),
            Key::Enter => super::save_selected_todo(),
            _ => (),
        })
        .text_signal(text_signal)
}

fn panel_footer() -> impl Element {
    let item_container = || El::new().s(Width::fill());
    Row::with_tag(Tag::Footer)
        .s(Padding::new().x(15).y(8))
        .s(Font::new().color(hsl(0, 0, 50)))
        .s(Borders::new().top(Border::new().color(hsl(0, 0, 91.3))))
        .s(Shadows::new(vec![
            Shadow::new().y(1).blur(1).color(hsla(0, 0, 0, 20)),
            Shadow::new().y(8).spread(-3).color(hsl(0, 0, 96.9)),
            Shadow::new()
                .y(9)
                .blur(1)
                .spread(-3)
                .color(hsla(0, 0, 0, 20)),
            Shadow::new().y(16).spread(-6).color(hsl(0, 0, 96.9)),
            Shadow::new()
                .y(17)
                .blur(2)
                .spread(-6)
                .color(hsla(0, 0, 0, 20)),
        ]))
        .item(item_container().child(active_items_count()))
        .item(item_container().child(filters()))
        .item(
            item_container()
                .child_signal(super::completed_exist().map_true(clear_completed_button)),
        )
}

fn active_items_count() -> impl Element {
    Text::with_signal(
        super::active_count()
            .map(|count| format!("{} item{} left", count, if count == 1 { "" } else { "s" })),
    )
}

fn filters() -> impl Element {
    Row::new()
        .s(Spacing::new(10))
        .items(Filter::iter().map(filter))
}

fn filter(filter: Filter) -> impl Element {
    let (label, route) = match filter {
        Filter::All => ("All", Route::Root),
        Filter::Active => ("Active", Route::Active),
        Filter::Completed => ("Completed", Route::Completed),
    };
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let is_hovered_selected = map_ref! {
        let hovered = hovered_signal,
        let selected = super::is_filter_selected(filter) =>
        (*hovered, *selected)
    };
    Button::new()
        .s(Padding::new().x(7).y(3))
        .s(Borders::all_signal(is_hovered_selected.map(
            |(hovered, selected)| {
                let border_alpha = if selected {
                    20
                } else if hovered {
                    10
                } else {
                    0
                };
                Border::new().color(hsla(12.2, 72.8, 40.2, border_alpha))
            },
        )))
        .s(RoundedCorners::all(3))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || router().go(route))
        .label(label)
}

fn clear_completed_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().right())
        .s(Font::new().underline_signal(hovered_signal))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::remove_completed_todos)
        .label("Clear completed")
}

fn footer() -> impl Element {
    Column::with_tag(Tag::Footer)
        .s(Spacing::new(9))
        .s(Font::new().size(10).color(hsl(0, 0, 77.3)).center())
        .item(Paragraph::new().content("Double-click to edit a todo"))
        .item(
            Paragraph::new()
                .content("Created by ")
                .content(author_link()),
        )
        .item(Paragraph::new().content("Part of ").content(todomvc_link()))
}

fn author_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new().underline_signal(hovered_signal))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Martin Kavík")
        .to("https://github.com/MartinKavik")
        .new_tab()
}

fn todomvc_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new().underline_signal(hovered_signal))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("TodoMVC")
        .to("http://todomvc.com")
        .new_tab()
}
