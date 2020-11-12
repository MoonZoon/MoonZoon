use zoon::*;
use std::ops::Not;

blocks!{

    #[el]
    pub fn root() -> View {
        view![
            font::size(14),
            font::family!("Helvetica Neue", "Helvetica", "Arial", font::sans_serif()),
            font::color(hsl(0, 0, 5.1))
            background::color(hsl(0, 0, 96.5)),
            column![
                width!(fill(), minimum(230), maximum(550)),
                center_x(),
                header(),
                panel(),
                footer(),
            ]
        ]
    }

    #[el]
    fn header() -> El {
        el![
            region::header(),
            width!(fill()),
            padding!(top(35), bottom(32)),
            el![
                region::h1(),
                center_x(),
                font::size(80),
                font::color(hsl(10.5, 62.8, 44.5)),
                font::extra_light(),
                "todos",
            ],
        ]
    }

    #[el]
    fn panel() -> Column {
        let todos_exist = super::todos_exist().inner();
        column![
            region::section(),
            width!(fill()),
            background::color(hsl(0, 0, 100)),
            border::shadow!(
                shadow::offsetXY(0, 2),
                shadow::size(0),
                shadow::blur(4),
                shadow::color(hsla(0, 0, 0, 20)),
            ),
            border::shadow!(
                shadow::offsetXY(0, 25),
                shadow::size(0),
                shadow::blur(50),
                shadow::color(hsla(0, 0, 0, 10)),
            ),
            panel_header(),
            todos_exist().then(|| elements![
                todos(),
                panel_footer(),
            ]),
        ]
    }

    #[el]
    fn panel_header() -> Row {
        let todos_exist = super::todos_exist().inner();
        row![
            width!(fill()),
            background::color(hsla(0, 0, 0, 0.3)),
            padding!(16),
            border::shadow!(
                shadow::inner(),
                shadow::offsetXY(-2, 1),
                shadow::size(0),
                shadow::color(hsla(0, 0, 0, 3)),
            ),
            todos_exist.then(toggle_all_checkbox),
            new_todo_title(),
        ]
    }

    #[el]
    fn toggle_all_checkbox() -> Checkbox {
        let checked = super::are_all_completed().inner();
        checkbox![
            checkbox::checked(checked),
            checkbox::on_change(super::check_or_uncheck_all),
            input::label_hidden("Toggle All"),
            el![
                font::color(hsla(0, 0, if checked { 48.4 } else { 91.3 })),
                font::size(22),
                rotate(90),
                "❯",
            ],
        ]
    }

    #[el]
    fn new_todo_title() -> TextInput {
        let new_todo_title = super::new_todo_title().inner();
        text_input![
            do_once(focus),
            text_input::on_change(super::set_new_todo_title),
            input::label_hidden("New Todo Title"),
            placeholder![
                font::italic(),
                font::light(),
                font::color(hsla(0, 0, 0, 40)),
                placeholder::text("what needs to be done?"),
            ],
            new_todo_title,
        ]
    }

    #[el]
    fn todos() -> Column {
        let filtered_todo = super::filtered_todos().inner();
        column![
            filtered_todo.map(|todos| todos.iter().rev().map(todo))
        ]
    }

    fn active_todo_checkbox_icon() -> &'static str {
        "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E"
    }

    fn completed_todo_checkbox_icon() -> &'static str {
        "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E"
    }

    #[el]
    fn todo(todo: Var<super::Todo>) -> Row {
        let selected = Some(todo) == super::selected_todo();
        let checkbox_id = el_var(ElementId::new);
        let row_hovered = el_var(|| false);
        row![
            font::size(24),
            padding!(15),
            spacing(10),
            on_hovered_change(row_hovered.setter()),
            todo_checkbox(checkbox_id, todo),
            selected.not().then(|| todo_label(checkbox_id, todo)),
            selected.then(selected_todo_title),
            row_hovered.inner().then(|| remove_todo_button(todo)),
        ]
    }

    #[el]
    fn todo_checkbox(checkbox_id: ElVar<ElementId>, todo: Var<super::Todo>) -> CheckBox {
        let completed = todo.try_map(|todo| todo.completed).unwrap_or_default();
        checkbox![
            id(checkbox_id.inner()),
            checkbox::checked(completed),
            checkbox::on_change(|_| super::toggle_todo(todo)),
            el![
                background::image(if completed {
                    completed_todo_checkbox_icon()
                } else {
                    active_todo_checkbox_icon()
                }),
            ],
        ]
    }

    #[el]
    fn todo_label(checkbox_id: ElVar<ElementId>, todo: Var<super::Todo>) -> Label {
        label![
            label::for_input(checkbox_id.inner()),
            checked.then(font::strike),
            font::regular(),
            font::color(hsl(0, 0, 32.7)),
            on_double_click(|| select_todo(Some(todo))),
            todo.try_map(|todo| todo.title.clone()),
        ]
    }

    #[el]
    fn selected_todo_title() -> TextInput {
        let selected_todo = super::selected_todo().inner().expect("selected todo");
        text_input![
            width!(fill()),
            paddingXY(16, 12),
            border::solid(),
            border::width!(1),
            border::color(hsl(0, 0, 63.2)),
            border::shadow!(
                shadow::inner(),
                shadow::offsetXY(-1, 5),
                shadow::size(0),
                shadow::color(hsla(0, 0, 0, 20)),
            ),
            do_once(focus),
            on_blur(super::save_selected_todo),
            on_key_down(|event| {
                match event.key {
                    Key::Escape => super::select_todo(None),
                    Key::Enter => super::save_selected_todo(),
                    _ => (),
                }
            }),
            text_input::on_change(super::set_selected_todo_title),
            selected_todo.try_map(|todo| todo.title.clone()),
        ]
    }

    #[el]
    fn remove_todo_button(todo: Var<super::Todo>) -> Button {
        let hovered = el_var(|| false);
        button![
            size::width!(20),
            size::height!(20),
            font::size(30),
            font::color(hsl(12.2, 34.7, 68.2)),
            on_hovered_change(hovered.setter()),
            font::color(if hovered().inner() { hsl(10.5, 37.7, 48.8) } else { hsl(12.2, 34.7, 68.2) }),
            button::on_press(|| super::remove_todo(todo)),
            "×",
        ]
    }

    #[el]
    fn panel_footer() -> Row {
        let completed_exist = super::completed_exist();
        row![
            active_items_count(),
            filters(),
            completed_exist.then(clear_completed_button),
        ]
    }

    #[el]
    fn active_items_count() -> Paragraph {
        let active_count = super::active_count().inner();
        paragraph![
            el![
                font::bold(),
                active_count,
            ],
            format!(" item{} left", if active_count == 1 { "" } else { "s" }),
        ]
    }

    #[el]
    fn filters() -> Row {
        let filters = super::filters();
        row![
            filters.map(|filters| filters.iter().map(filter))  
        ]
    }

    #[el]
    fn filter(filter: super::Filter) -> Button {
        let selected = super::selected_filter().inner() == filter;
        let hovered = el_var(|| false);
        let (title, route) = match filter {
            super::Filter::All => ("All", super::Route::root()),
            super::Filter::Active => ("Active", super::Route::active()),
            super::Filter::Completed => ("Completed", super::Route::completed()),
        };
        let border_alpha = if selected { 20 } else if hovered { 10 } else { 0 };
        button![
            on_hovered_change(hovered.setter()),
            paddingXY(7, 3),
            border::solid(),
            border::width!(1),
            border::color(hsla(12.2, 72.8, 40.2, border_alpha)),
            button::on_press(|| super::set_route(route)),
            title,
        ]
    }

    #[el]
    fn clear_completed_button() -> Button {
        let hovered = el_var(|| false);
        button![
            on_hovered_change(hovered.setter()),
            hovered.inner().then(font::underline),
            button::on_press(super::remove_completed),
            "Clear completed",
        ]
    }

    #[el]
    fn footer() -> Column {
        column![
            paragraph![
                "Double-click to edit a todo",
            ],
            paragraph![
                "Created by ",
                link![
                    link::new_tab(),
                    link::url("https://github.com/MartinKavik"),
                    "Martin Kavík",
                ],
            ],
            paragraph![
                "Part of ",
                link![
                    link::new_tab(),
                    link::url("http://todomvc.com"),
                    "TodoMVC",
                ],
            ],
        ]
    }

}
