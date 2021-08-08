use zoon::*;
use super::{Todo, Filter};
use crate::router::{router, Route};
use std::sync::Arc;
use strum::IntoEnumIterator;

pub fn root() -> impl Element {
    El::new()
        // font::size(14),
        // font::family!("Helvetica Neue", "Helvetica", "Arial", font::sans_serif()),
        // font::color(hsl(0, 0, 5.1))
        // background::color(hsl(0, 0, 96.5)),
        .child(
            Column::new()
                // width!(fill(), minimum(230), maximum(550)),
                // center_x(),
                .item(header())
                .item(panel())
                .item(footer())
        )
}

fn header() -> impl Element {
    El::new()
        // region::header(),
        // width!(fill()),
        // padding!(top(35), bottom(32)),
        .child(
            El::new()
                // region::h1(),
                // center_x(),
                // font::size(80),
                // font::color(hsl(10.5, 62.8, 44.5)),
                // font::extra_light(),
                .child("todos")
        )
}

fn panel() -> impl Element {
    Column::new()
        // region::section(),
        // width!(fill()),
        // background::color(hsl(0, 0, 100)),
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
        .item(panel_header())
        .item_signal(super::todos_exist().map_true(todos))
        .item_signal(super::todos_exist().map_true(panel_footer))
}

fn panel_header() -> impl Element {
    Row::new()
        //  width!(fill()),
        //  background::color(hsla(0, 0, 0, 0.3)),
        //  padding!(16),
        //  border::shadow!(
        //      shadow::inner(),
        //      shadow::offsetXY(-2, 1),
        //      shadow::size(0),
        //      shadow::color(hsla(0, 0, 0, 3)),
        //  ),
        .item_signal(super::todos_exist().map_true(toggle_all_checkbox))
        .item(new_todo_title())
}

fn toggle_all_checkbox() -> impl Element {
    Checkbox::new()
        .checked_signal(super::are_all_completed())
        // .on_click(super::check_or_uncheck_all_todos)
        .label_hidden("Toggle All")
        .icon(checkbox::default_icon)
        // .icon(|checked_signal| {
        //     El::new()
        //         .s(
        //             Font::new()
        //             .size(22)
        //             // font::color(hsla(0, 0, if checked { 48.4 } else { 91.3 })),
        //         )
        //         // rotate(90),
        //         .child(">")
        // })
}

fn new_todo_title() -> impl Element {
    TextInput::new()
        .focused()
        .on_change(super::set_new_todo_title)
        .label_hidden("New Todo Title")
        .placeholder(
            // font::italic(),
            // font::light(),
            // font::color(hsla(0, 0, 0, 40)),
            Placeholder::new("What needs to be done?")
        )
        .on_key_down(|event| event.if_key(Key::Enter, super::add_todo))
        .text_signal(super::new_todo_title().signal_cloned())
}

fn todos() -> impl Element {
    Column::new()
        .items_signal_vec(super::filtered_todos().map(todo))
}

fn todo(todo: Arc<Todo>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Row::new()
        .s(Font::new().size(24))
        .s(Padding::new().all(15))
        .s(Spacing::new(10))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .item(todo_checkbox(todo.clone()))
        .item_signal(super::is_todo_selected(todo.id).map_false(clone!((todo) move || todo_label(&todo))))
        .item_signal(super::is_todo_selected(todo.id).map_true(selected_todo_title))
        .item_signal(hovered_signal.map_true(move || remove_todo_button(&todo)))
}

fn todo_checkbox(todo: Arc<Todo>) -> impl Element {
    Checkbox::new()
        .id(todo.id.to_string())
        .checked_signal(todo.completed.signal())
        .on_change(move |checked| todo.completed.set_neq(checked))
        .icon(checkbox::default_icon)
        // .icon(|checked_signal| {
        //     El::new()
        //     // background::image(if completed {
        //     //     completed_todo_checkbox_icon()
        //     // } else {
        //     //     active_todo_checkbox_icon()
        //     // }),
        // })
}

fn active_todo_checkbox_icon() -> &'static str {
    "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E"
}

fn completed_todo_checkbox_icon() -> &'static str {
    "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E"
}

fn todo_label(todo: &Todo) -> impl Element {
    Label::new()
        // checked.then(font::strike),
        // font::regular(),
        // font::color(hsl(0, 0, 32.7)),
        // on_double_click(|| select_todo(Some(todo))),
        .for_input(todo.id.to_string())
        .label_signal(todo.title.signal_cloned())
}

fn selected_todo_title() -> impl Element {
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
        // on_blur(super::save_selected_todo),
        .on_change(super::set_selected_todo_title)
        .on_key_down(|event| match event.key() {
            Key::Escape => super::select_todo(None),
            Key::Enter => super::save_selected_todo(),
            _ => ()
        })
        .text_signal(super::selected_todo_title().signal_cloned().map(Option::unwrap_or_default))
}

fn remove_todo_button(todo: &Todo) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let id = todo.id;
    Button::new()
        // size::width!(20),
        // size::height!(20),
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
                .s(Font::new().bold())
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
        //  hovered.inner().then(font::underline),
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::remove_completed_todos)
        .label("Clear completed")
}

fn footer() -> impl Element {
    Column::new()
        .item(Paragraph::new().content("Double-click to edit a todo"))
        .item(
            Paragraph::new()
                .content("Created by ")
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
