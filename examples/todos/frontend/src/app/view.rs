use zoon::*;
use super::{Todo, Filter};
use crate::router::{router, Route};
use std::sync::Arc;
use strum::IntoEnumIterator;

pub fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::fill().min_screen())
        .s(
            Font::new()
                .size(14)
                .color(hsl(0, 0, 5.1))
                .weight(NamedWeight::Light)
                .family(vec![
                    FontFamily::new("Helvetica Neue"),
                    FontFamily::new("Helvetica"),
                    FontFamily::new("Arial"),
                    FontFamily::SansSerif,
                ])
        )
        .s(Background::new().color(hsl(0, 0, 96.5)))
        .item(content())
}

fn content() -> impl Element {
    Column::new()
        // region::header(),
        .s(Width::fill().min(230).max(550))
        .s(Align::new().center_x())
        .item(header())
        .item(
            Column::new()
                .s(Width::fill())
                .s(Spacing::new(65))
                .item(panel())
                .item(footer())
        )
}

fn header() -> impl Element {
    El::new()
        // region::h1(),
        .s(Padding::new().top(10))
        .s(Align::new().center_x())
        .s(Height::new(130))
        .s(Font::new()
            .size(100)
            .color(hsla(10.5, 62.8, 44.5, 15))
            .weight(NamedWeight::Hairline)
        )
        .child("todos")
}

fn panel() -> impl Element {
    Column::new()
        // region::section(),
        // border::shadow!(
        //     shadow::offsetXY(0, 2),
        //     shadow::size(0),
        //     shadow::blur(4),
        //     shadow::color(hsla(0, 0, 0, 20)),
        // ),
        // border::shadow!(
        //     shadow::offsetXY(0, 25),
        //     shadow::size(0),
        //     shadow::blur(50),
        //     shadow::color(hsla(0, 0, 0, 10)),
        // ),
        .s(Width::fill())
        .s(Background::new().color(hsl(0, 0, 100)))
        .item(new_todo_title())
        .item_signal(super::todos_exist().map_true(todos))
        .item_signal(super::todos_exist().map_true(panel_footer))
}

fn new_todo_title() -> impl Element {
    TextInput::new()
        .s(Padding::new().y(20).left(60).right(16))
        .s(Font::new().size(24))
        .s(Background::new().color(hsla(0, 0, 0, 0.3)))
        .focused()
        .on_change(super::set_new_todo_title)
        .label_hidden("What needs to be done?")
        .placeholder(
            Placeholder::new("What needs to be done?")
                // .s(
                //     Font::new()
                //         .weight(FontWeight::Bold)
                //         .italic(true)
                //         .color(hsla(0, 0, 0, 40))
                // )
        )
        .on_key_down(|event| event.if_key(Key::Enter, super::add_todo))
        .text_signal(super::new_todo_title().signal_cloned())
}

fn todos() -> impl Element {
    // [ width fill
    //     , transparent <| List.isEmpty entries
    //     , Border.widthEach { edges | top = 1 }
    //     , Border.solid
    //     , Border.color <| rgb255 230 230 230
    Column::new()
        .items_signal_vec(super::filtered_todos().map(todo))
        .add_above(toggle_all_checkbox())
}

fn toggle_all_checkbox() -> impl Element {
    Checkbox::new()
        .s(Width::new(60))
        .s(Height::fill())
        .checked_signal(super::are_all_completed().signal())
        .on_click(super::check_or_uncheck_all_todos)
        .label_hidden("Toggle all")
        .icon(|checked_signal| {
            El::new()
                .s(
                    Font::new()
                    .size(22)
                    .color_signal(checked_signal.map_bool(
                        || hsl(0, 0, 48.4), 
                        || hsl(0, 0, 91.3)
                    ))
                )
                .s(Transform::new().rotate(90).move_up(18))
                .s(Height::new(34))
                .s(Padding::new().x(27).y(10))
                .child("❯")
        })
}

fn todo(todo: Arc<Todo>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Row::new()
        .s(Font::new().size(24))
        .s(Padding::new().all(15))
        .s(Spacing::new(10))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .item(todo_checkbox(todo.clone()))
        .item_signal(super::is_todo_selected(todo.id).map_false(clone!((todo) move || todo_label(todo.clone()))))
        .item_signal(super::is_todo_selected(todo.id).map_true(clone!((todo) move || edited_todo_title(todo.clone()))))
        .item_signal(hovered_signal.map_true(move || remove_todo_button(&todo)))
}

fn todo_checkbox(todo: Arc<Todo>) -> impl Element {
    static ACTIVE_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E";
    static COMPLETED_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E";
    
    Checkbox::new()
        .id(todo.id.to_string())
        .checked_signal(todo.completed.signal())
        .on_change(move |checked| todo.completed.set_neq(checked))
        .icon(|checked_signal| {
            El::new()
                .s(Width::new(40))
                .s(Height::new(40))
                .s(Background::new().url_signal(checked_signal.map_bool(
                    || COMPLETED_ICON, || ACTIVE_ICON
                )))
        })
}

fn todo_label(todo: Arc<Todo>) -> impl Element {
    Label::new()
        .s(Font::new()
            .color_signal(todo.completed.signal().map_bool(
                || hsl(0, 0, 86.7),
                || hsl(0, 0, 32.7)
            ))
            .strike_signal(todo.completed.signal())
        )
        .for_input(todo.id.to_string())
        .label_signal(todo.title.signal_cloned())
        .on_double_click(move || super::select_todo(Some(todo)))
}

fn edited_todo_title(todo: Arc<Todo>) -> impl Element {
    let text_signal = todo.edited_title.signal_cloned().map(Option::unwrap_throw);
    TextInput::new()
        .s(Padding::new().x(16).y(12))
        // width!(fill()),
        // border::solid(),
        // border::width!(1),
        // border::color(hsl(0, 0, 63.2)),
        // border::shadow!(
        //     shadow::inner(),
        //     shadow::offsetXY(-1, 5),
        //     shadow::size(0),
        //     shadow::color(hsla(0, 0, 0, 20)),
        // ),
        .label_hidden("selected todo title")
        .focused()
        .on_blur(super::save_selected_todo)
        .on_change(move |text| todo.edited_title.set_neq(Some(text)))
        .on_key_down(|event| match event.key() {
            Key::Escape => super::select_todo(None),
            Key::Enter => super::save_selected_todo(),
            _ => ()
        })
        .text_signal(text_signal)
}

fn remove_todo_button(todo: &Todo) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = todo.id;
    Button::new()
        .s(Width::new(20))
        .s(Height::new(20))
        .s(Font::new().size(30))
        // font::color(if hovered().inner() { hsl(10.5, 37.7, 48.8) } else { hsl(12.2, 34.7, 68.2) }),
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || super::remove_todo(id))
        .label("x")
}

fn panel_footer() -> impl Element {
    Row::new()
        .item(active_items_count())
        .item(filters())
        .item_signal(super::completed_exist().map_true(clear_completed_button))
}

fn active_items_count() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().weight(NamedWeight::Bold))
                .child(Text::with_signal(super::active_count()))
        )
        .content(Text::with_signal(super::active_count().map(|count| {
            format!(" item{} left", if count == 1 { "" } else { "s" })
        })))
}

fn filters() -> impl Element {
    Row::new()
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
    // let border_alpha = if selected { 20 } else if hovered { 10 } else { 0 };
    Button::new()
        .s(Padding::new().x(7).y(3))
        // border::solid(),
        // border::width!(1),
        // border::color(hsla(12.2, 72.8, 40.2, border_alpha)),
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || router().go(route))
        .label(label)
}

fn clear_completed_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Font::new().underline_signal(hovered_signal))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::remove_completed_todos)
        .label("Clear completed")
}

fn footer() -> impl Element {
    Column::new()
        // region::footer
        .s(Spacing::new(9))
        .s(
            Font::new()
                .size(10)
                .color(hsl(0, 0, 77.3))
                .center()
        )
        .item(Paragraph::new().content("Double-click to edit a todo"))
        .item(
            Paragraph::new()
                .content("Written by ")
                .content(author_link())
        )
        .item(
            Paragraph::new()
                .content("Part of ")
                .content(todomvc_link())
        )
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
