use zoon::*;
use crate::model;

#[View]
pub fn view() -> View {
    view![
        font::size(14),
        font::family!("Helvetica Neue", "Helvetica", "Arial", font::sans_serif()),
        font::color(hsl(0, 0, 5.1))
        background::color(hsl(0, 0, 96.5)),
        column![
            width!(fill(), minimum(230), maximum(550)),
            center_x(),
            header(),
            main(),
            footer(),
        ]
    ]
}

#[View]
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

#[View]
fn main() -> Column {
    column![
        region::section(),
        width!(fill()),
        background::color(hsl(0, 0, 100)),
        border::shadow!(
            shadow::offsetXY(0, 2),
            shadow::size(0),
            shadow::blur(4),
            shadow::color(hsla(0, 0, 0, 0.2)),
        ),
        border::shadow!(
            shadow::offsetXY(0, 25),
            shadow::size(0),
            shadow::blur(50),
            shadow::color(hsla(0, 0, 0, 0.1)),
        ),
        row![
            width!(fill()),
            background::color(hsla(0, 0, 0, 0.003)),
            padding!(16),
            border::inner_shadow!(
                shadow::offsetXY(-2, 1),
                shadow::size(0),
                shadow::color(hsla(0, 0, 0, 0.03)),
            ),
            app::todos_exist().map_true(toggle_all_checkbox),
            new_todo_title(),
        ],
        app::todos_exist().map_true(|| elements![
            todos(),
            status_bar(),
        ]),
    ]
}

#[View]
fn toggle_all_checkbox() -> Checkbox {
    let checked = app::are_all_completed().inner();
    checkbox![
        checkbox::checked(checked),
        checkbox::on_change(app::check_or_uncheck_all),
        input::label_hidden("Toggle All"),
        el![
            font::color(hsla(0, 0, if checked { 48.4 } else { 91.3 })),
            font::size(22),
            rotate(90),
            "❯",
        ],
    ]
}

#[View]
fn new_todo_title() -> TextInput {
    text_input![
        focus_on_load(),
        text_input::on_change(app::set_new_todo_title),
        input::label_hidden("New Todo Title"),
        placeholder![
            font::italic(),
            font::light(),
            font::color(hsla(0, 0, 0, 0.4)),
            placeholder::text("what needs to be done?"),
        ],
        app::new_todo_title().inner(),
    ]
}

#[View]
fn todos() -> Column {
    column![
        app::filtered_todos().iter().map(todo)
    ]
}

fn active_todo_checkbox_icon() -> &'static str {
    "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E"
}

fn completed_todo_checkbox_icon() -> &'static str {
    "data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E"
}

#[View]
fn todo(todo: Model<Todo>) -> Row {
    let selected_todo = app::selected_todo();
    let selected = todo.map(|t| t.id) == selected_todo.map(|t| t.id);
    let completed = todo.map(|t| t.completed);

    let checkbox_id = use_state(ElementId::new);
    let row_hovered = use_state(|| false);

    row![
        font::size(24),
        padding!(15),
        spacing(10),
        hovered(row_hovered),
        checkbox![
            id(checkbox_id.inner()),
            checkbox::checked(completed),
            checkbox::on_change(|_| app::toggle_todo(todo)),
            el![
                background::image(if completed {
                    completed_todo_checkbox_icon()
                } else {
                    active_todo_checkbox_icon()
                }),
            ],
        ],
        if selected {
            text_input![
                width!(fill()),
                paddingXY(16, 12),
                border::solid(),
                border::width!(1),
                border::color(hsl(0, 0, 63.2)),
                border::inner_shadow!(
                    shadow::offsetXY(-1, 5),
                    shadow::size(0),
                    shadow::color(hsla(0, 0, 0, 0.2)),
                ),
                todo.map(|t| t.title.clone()),
            ].into_element()
        } else {
            label![
                label::for_input(checkbox_id.inner()),
                checked.map_true(font::strike),
                font::regular(),
                font::color(hsl(0, 0, 32.7)),
                todo.map(|t| t.title.clone()),
            ].into_element()
        },
        row_hovered.inner().map_true(|| remove_todo_button(todo)),
    ]
}

#[View]
fn remove_todo_button(todo: Model<Todo>) -> Button {
    let hovered = use_state(|| false);
    button![
        size::width!(20),
        size::height!(20),
        font::size(30),
        font::color(hsl(12.2, 34.7, 68.2)),
        hovered(hovered),
        button::on_press(|| app::remove_todo(todo)),
        hovered.inner().map_true(|| font::color(hsl(10.5, 37.7, 48.8))),
        "×",
    ]
}

#[View]
fn status_bar() -> Row {
    C.text(app::active_count())
    filters()
    if app::completed_exist() {
        C.button("Clear completed")
    }
}

#[View]
fn filters() -> Row {
    app::filters().iter().map(filter)  
}

#[View]
fn filter(filter: Filter) -> Button {
    let selected_filter = app::selected_filter();
    C.button(is_selected)
}

#[View]
fn footer() -> Column {
    
}
