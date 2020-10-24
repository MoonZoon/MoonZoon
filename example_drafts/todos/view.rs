#[View]
fn view() {
    header()
    if model::todos_exist() {
        main()
        footer()
    }
}

#[View]
fn header() {
    C.title("todos")
    C.text_input(model::new_todo_title())
}

#[View]
fn main() {
    toggle_all_checkbox()
    todos()
}

#[View]
fn toggle_all_checkbox() {
    C.checkbox(model::are_all_completed())
}

#[View]
fn todos() {
    model::filtered_todos().iter().map(todo)
}

#[View]
fn todo(todo: State<Todo>) {
    let selected_todo = model::selected_todo();
    C.checkbox(is_selected, is_completed, ..)
    if is_selected {
        C.text_input
    }
}

#[View]
fn footer() {
    C.text(model::active_count())
    filters()
    if model::completed_exist() {
        C.button("Clear completed")
    }
}

#[View]
fn filters() {
    model::filters().iter().map(filter)  
}

#[View]
fn filter(filter: Filter) {
    let selected_filter = model::selected_filter();
    C.button(is_selected)
}
