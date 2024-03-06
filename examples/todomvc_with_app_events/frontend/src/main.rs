use std::sync::Arc;
use zoon::{strum::IntoEnumIterator, *};

mod app_event;
mod store;

use app_event::*;
use store::*;

fn selected_filter() -> impl Signal<Item = Filter> {
    ROUTER.route().signal_ref(|route| match route {
        NoRoute | UnknownRoute | KnownRoute(Route::Root) => Filter::All,
        KnownRoute(Route::Active) => Filter::Active,
        KnownRoute(Route::Completed) => Filter::Completed,
    })
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::fill().min_screen())
        .s(Font::new()
            .size(14)
            .color(color!("oklch(17.76% 0 0)"))
            .weight(FontWeight::Light)
            .family([
                FontFamily::new("Helvetica Neue"),
                FontFamily::new("Helvetica"),
                FontFamily::new("Arial"),
                FontFamily::SansSerif,
            ]))
        .s(Background::new().color(color!("oklch(97.02% 0 0)")))
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
                .s(Gap::both(65))
                .item(panel())
                .item(footer()),
        )
}

fn header() -> impl Element {
    El::with_tag(Tag::Header)
        .s(Padding::new().top(10))
        .s(Align::new().center_x())
        .s(Height::exact(130))
        .s(Font::new()
            .size(100)
            .color(color!("oklch(54.04% 0.156 21.24 / 15%)"))
            .weight(FontWeight::Hairline))
        .child(El::with_tag(Tag::H1).child("todos"))
}

fn panel() -> impl Element {
    Column::with_tag(Tag::Section)
        .s(Shadows::new([
            Shadow::new()
                .y(2)
                .blur(4)
                .color(color!("oklch(0% 0 0 / 20%)")),
            Shadow::new()
                .y(25)
                .blur(50)
                .color(color!("oklch(0% 0 0 / 10%)")),
        ]))
        .s(Width::fill())
        .s(Background::new().color(color!("oklch(100% 0 0)")))
        .item(new_todo_title())
        .item_signal(TODOS.signal_vec_cloned().is_empty().map_false(todos))
        .item_signal(TODOS.signal_vec_cloned().is_empty().map_false(panel_footer))
}

fn new_todo_title() -> impl Element {
    TextInput::new()
        .s(Padding::new().y(19).left(60).right(16))
        .s(Font::new().size(24).color(color!("oklch(42.02% 0 0)")))
        .s(Background::new().color(color!("oklch(0% 0 0 / 0.3%)")))
        .s(Shadows::new([Shadow::new()
            .inner()
            .y(-2)
            .blur(1)
            .color(color!("oklch(0% 0 0 / 3%)"))]))
        .focus(true)
        .on_change(|title| emit(NewTodoTitleChanged { title }))
        .label_hidden("What needs to be done?")
        .placeholder(
            Placeholder::new("What needs to be done?")
                .s(Font::new().italic().color(color!("oklch(92.49% 0 0)"))),
        )
        .on_key_down_event(|event| {
            event.if_key(Key::Enter, || {
                let new_todo_title = NEW_TODO_TITLE.lock_ref();
                let trimmed_title = new_todo_title.trim();
                if not(trimmed_title.is_empty()) {
                    emit(NewTodoTitlePreparedForSaving {
                        title: Arc::new(trimmed_title.to_owned()),
                    });
                }
            })
        })
        .text_signal(NEW_TODO_TITLE.signal_cloned())
}

fn todos() -> impl Element {
    Column::new()
        .s(Borders::new().top(Border::new().color(color!("oklch(92.49% 0 0)"))))
        .s(Background::new().color(color!("oklch(94.61% 0 0)")))
        .s(Gap::both(1))
        .items_signal_vec(
            TODOS
                .signal_vec_cloned()
                .filter_signal_cloned(|todo| {
                    map_ref! {
                        let completed = todo.completed.signal(),
                        let filter = selected_filter() => match filter {
                            Filter::All => true,
                            Filter::Active => not(*completed),
                            Filter::Completed => *completed,
                        }
                    }
                })
                .map(todo),
        )
        .element_above(toggle_all_checkbox())
}

fn toggle_all_checkbox() -> impl Element {
    let are_all_todos_completed = TODOS
        .signal_vec_cloned()
        .filter_signal_cloned(|todo| signal::not(todo.completed.signal()))
        .is_empty();
    Checkbox::new()
        .s(Width::exact(60))
        .s(Height::fill())
        .checked_signal(are_all_todos_completed)
        .on_click(|| emit(ToggleAllCheckboxClicked))
        .label_hidden("Toggle all")
        .icon(|checked| {
            El::new()
                .s(Font::new().size(22).color_signal(checked.signal().map_bool(
                    || color!("oklch(55.55% 0 0)"),
                    || color!("oklch(92.49% 0 0)"),
                )))
                .s(Transform::new().rotate(90).move_up(18))
                .s(Height::exact(34))
                .s(Padding::new().x(27).y(6))
                .child("❯")
        })
}

fn todo(todo: Todo) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Background::new().color(color!("oklch(100% 0 0)")))
        .s(Gap::both(5))
        .s(Font::new().size(24))
        .items_signal_vec(
            SELECTED_TODO
            .signal_ref(move |selected_todo| matches!(selected_todo, Some(selected_todo) if selected_todo.id == todo.id))
            .dedupe_map(move |is_selected| { if *is_selected {
                    element_vec![editing_todo_title(todo.clone())]
                } else {
                    element_vec![todo_checkbox(todo.clone()), todo_title(todo.clone())]
                }
            })
            .to_signal_vec()
        )
}

fn todo_checkbox(todo: Todo) -> impl Element {
    static ACTIVE_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E";
    static COMPLETED_ICON: &str = "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E";

    Checkbox::new()
        .id(todo.id.to_string())
        .checked_signal(todo.completed.signal())
        .on_change(move |checked| {
            emit(TodoCheckboxChanged {
                todo: todo.clone(),
                checked,
            })
        })
        .icon(|checked| {
            El::new()
                .s(Width::exact(40))
                .s(Height::exact(40))
                .s(Background::new()
                    .url_signal(checked.signal().map_bool(|| COMPLETED_ICON, || ACTIVE_ICON)))
        })
}

fn todo_title(todo: Todo) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Label::new()
        .s(Width::fill())
        .s(Font::new()
            .color_signal(
                todo.completed
                    .signal()
                    .map_bool(|| color!("oklch(88.53% 0 0)"), || color!("oklch(42.02% 0 0)")),
            )
            .size(24)
            .line(FontLine::new().strike_signal(todo.completed.signal())))
        .s(Padding::all(15).right(60))
        .s(Clip::x())
        .for_input(todo.id.to_string())
        .label_signal(todo.title.signal_cloned())
        .on_double_click(clone!((todo) move || emit(TodoTitleDoubleClicked { todo: todo.clone(), title: todo.title.get_cloned() })))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .element_on_right_signal(hovered_signal.map_true(move || remove_todo_button(todo.clone())))
}

fn remove_todo_button(todo_to_remove: Todo) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(40))
        .s(Height::exact(40))
        .s(Transform::new().move_left(50).move_down(14))
        .s(Font::new()
            .size(30)
            .center()
            .color_signal(hovered_signal.map_bool(
                || color!("oklch(57.15% 0.109 18.87)"),
                || color!("oklch(73.32% 0.06 18.62)"),
            )))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || {
            emit(RemoveTodoButtonPressed {
                todo: todo_to_remove.clone(),
            })
        })
        .label("×")
}

fn editing_todo_title(todo: Todo) -> impl Element {
    TextInput::new()
        .s(Width::exact(506))
        .s(Padding::all(17).bottom(16))
        .s(Align::new().right())
        .s(Borders::all(
            Border::new().color(color!("oklch(68.3% 0 0)")),
        ))
        .s(Shadows::new([Shadow::new()
            .inner()
            .y(-1)
            .blur(5)
            .color(color!("oklch(0% 0 0 / 20%)"))]))
        .s(Font::new().color(color!("oklch(42.02% 0 0)")))
        .text_signal(todo.edited_title.signal_cloned())
        .label_hidden("selected todo title")
        .focus(true)
        .on_blur(
            clone!((todo) move || emit(EditingTodoTitleBlurredOrEnterPressed {
                todo: todo.clone(),
                edited_title: todo.edited_title.get_cloned().unwrap_throw()
            })),
        )
        .on_change(clone!((todo) move |text| {
            emit(EditingTodoTitleChanged {
                todo: todo.clone(),
                text,
            })
        }))
        .on_key_down_event(move |event| match event.key() {
            Key::Escape => emit(EditingTodoTitleEscapePressed),
            Key::Enter => emit(EditingTodoTitleBlurredOrEnterPressed {
                todo: todo.clone(),
                edited_title: todo.edited_title.get_cloned().unwrap_throw(),
            }),
            _ => (),
        })
}

fn panel_footer() -> impl Element {
    let item_container = || El::new().s(Width::fill());
    Row::with_tag(Tag::Footer)
        .s(Padding::new().x(15).y(8))
        .s(Font::new().color(color!("oklch(56.93% 0 0)")))
        .s(Borders::new().top(Border::new().color(color!("oklch(92.49% 0 0)"))))
        .s(Shadows::new([
            Shadow::new()
                .y(1)
                .blur(1)
                .color(color!("oklch(0% 0 0 / 20%)")),
            Shadow::new()
                .y(8)
                .spread(-3)
                .color(color!("oklch(97.31% 0 0)")),
            Shadow::new()
                .y(9)
                .blur(1)
                .spread(-3)
                .color(color!("oklch(0% 0 0 / 20%)")),
            Shadow::new()
                .y(16)
                .spread(-6)
                .color(color!("oklch(97.31% 0 0)")),
            Shadow::new()
                .y(17)
                .blur(2)
                .spread(-6)
                .color(color!("oklch(0% 0 0 / 20%)")),
        ]))
        .item(item_container().child(active_items_count()))
        .item(item_container().child(filters()))
        .item(
            item_container().child_signal(
                TODOS
                    .signal_vec_cloned()
                    .filter_signal_cloned(|todo| todo.completed.signal())
                    .is_empty()
                    .map_false(clear_completed_button),
            ),
        )
}

fn active_items_count() -> impl Element {
    Text::with_signal(
        TODOS
            .signal_vec_cloned()
            .filter_signal_cloned(|todo| signal::not(todo.completed.signal()))
            .len()
            .map(|count| format!("{count} item{} left", if count == 1 { "" } else { "s" })),
    )
}

fn filters() -> impl Element {
    Row::new()
        .s(Gap::both(10))
        .items(Filter::iter().map(filter))
}

fn filter(filter: Filter) -> impl Element {
    let (label, route) = match filter {
        Filter::All => ("All", Route::Root),
        Filter::Active => ("Active", Route::Active),
        Filter::Completed => ("Completed", Route::Completed),
    };
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let outline_alpha = map_ref! {
        let hovered = hovered_signal,
        let selected = selected_filter().map(move |selected_filter| selected_filter == filter) =>
        if *selected {
            Some(0.2)
        } else if *hovered {
            Some(0.1)
        } else {
            None
        }
    };
    Button::new()
        .s(Padding::new().x(8).y(4))
        .s(Outline::with_signal(outline_alpha.map_some(|alpha| {
            Outline::inner().color(color!("oklch(50.4% 0.165 25.36)").a(alpha))
        })))
        .s(RoundedCorners::all(3))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || emit(FilterPressed { route }))
        .label(label)
}

fn clear_completed_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().right())
        .s(Font::new().line(FontLine::new().underline_signal(hovered_signal)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(|| emit(ClearCompletedButtonPressed))
        .label("Clear completed")
}

fn footer() -> impl Element {
    Column::with_tag(Tag::Footer)
        .s(Gap::both(9))
        .s(Font::new()
            .size(10)
            .color(color!("oklch(80.47% 0 0)"))
            .center())
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
        .s(Font::new().line(FontLine::new().underline_signal(hovered_signal)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Martin Kavík")
        .to("https://github.com/MartinKavik")
        .new_tab(NewTab::new())
}

fn todomvc_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new().line(FontLine::new().underline_signal(hovered_signal)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("TodoMVC")
        .to("http://todomvc.com")
        .new_tab(NewTab::new())
}
