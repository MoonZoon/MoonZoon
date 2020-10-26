use zoon::*;
use crate::model;

#[View]
pub fn view() -> View {
    view![
        font::size(14),
        font::family![
            font::typeface("Helvetica Neue"),
            font::typeface("Helvetica"),
            font::typeface("Arial"),
            font::sans_serif(),
        ],
        font::color(hsl(0, 0, 5.1))
        background::color(hsl(0, 0, 96.5)),
        column![
            size::width![
                size::fill(), 
                size::minimum(230),
                size::maximum(550),
            ],
            align::center_x(),
            header(),
            main(),
            app::todos_exist().map_true(footer),
        ]
    ]
}

#[View]
fn header() -> El {
    el![
        region::header(),
        size::width![size::fill()],
        padding::top(35),
        padding::bottom(32),
        el![
            region::h1(),
            align::center_x(),
            font::size(80),
            font::color(hsl(10.5, 62.8, 44.5)),
            font::extra_light(),
            text!["todos"],
        ],
    ]
}

#[View]
fn main() -> Column {
    column![
        region::section(),
        size::width![size::fill()],
        background::color(hsl(0, 0, 100)),
        border::shadow![
            shadow::offsetXY(0, 2),
            shadow::size(0),
            shadow::blur(4),
            shadow::color(hsla(0, 0, 0, 0.2)),
        ],
        border::shadow![
            shadow::offsetXY(0, 25),
            shadow::size(0),
            shadow::blur(50),
            shadow::color(hsla(0, 0, 0, 0.1)),
        ],
        row![
            size::width![size::fill()],
            background::color(hsla(0, 0, 0, 0.003)),
            padding::all(16),
            border::inner_shadow![
                shadow::offsetXY(-2, 1),
                shadow::size(0),
                shadow::color(hsla(0, 0, 0, 0.03)),
            ],
            app::todos_exist().map_true(toggle_all_checkbox),
            new_todo_title(),
        ],
        app::todos_exist().map_true(todos),
    ]
}

#[View]
fn toggle_all_checkbox() -> Checkbox {
    checkbox![
        checkbox::checked(app::are_all_completed()),
        checkbox::on_change(app::check_or_uncheck_all()),
        checkbox::icon(|checked| el![
            font::color(hsla(0, 0, if checked { 48.4 } else { 91.3 })),
            font::size(22),
            transform::rotate(90),
            text!("❯"),
        ]),
        label![
            label::text("Toggle All"),
        ]
    ]
}

fn new_todo_title() -> TextInput {
    text_input![
        focus::on_load(),
        text_input::text(app::new_todo_title().inner()),
        text_input::on_change(app::set_new_todo_title),
        placeholder![
            font::italic(),
            font::light(),
            font::color(hsla(0, 0, 0, 0.4)),
            placeholder::text("what needs to be done?"),
        ],
        label![
            label::text("New Todo Title"),
        ],
    ]
}

#[View]
fn todos() {
    app::filtered_todos().iter().map(todo)
}

#[View]
fn todo(todo: State<Todo>) {
    let selected_todo = app::selected_todo();
    C.checkbox(is_selected, is_completed, ..)
    if is_selected {
        C.text_input
    }
}

#[View]
fn footer() {
    C.text(app::active_count())
    filters()
    if app::completed_exist() {
        C.button("Clear completed")
    }
}

#[View]
fn filters() {
    app::filters().iter().map(filter)  
}

#[View]
fn filter(filter: Filter) {
    let selected_filter = app::selected_filter();
    C.button(is_selected)
}
