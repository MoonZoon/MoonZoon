use zoon::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ulid::Ulid;

const STORAGE_KEY: &str = "todos-zoon";

type TodoId = Ulid;

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route("active")]
    Active,
    #[route("completed")]
    Completed,
    #[route()]
    Root,
    Unknown,
}

#[cache]
fn route() -> Route {
    zoon::model::url().map(Route::from)
}

#[update]
fn set_route(route: Route) {
    zoon::model::url().set(Url::from(route))
}

// ------ Filters ------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
enum Filter {
    All,
    Active,
    Completed,
}

#[model]
fn filters() -> Vec<Filter> {
    Filter::iter().collect()
}

#[cache]
fn selected_filter() -> Filter {
    match route().inner() {
        Route::Active => Filter::Active,
        Route::Completed => Filter::Completed,
        _ => Filter::All,
    }
}

// ------ SelectedTodo ------

#[model]
fn selected_todo() -> Option<Model<Todo>> {
    None
}

#[update]
fn select_todo(todo: Option<Model<Todo>>) {
    selected_todo().set(todo)
}

#[model]
fn selected_todo_title() -> Option<String> {
    selected_todo().map(|todo| todo?.try_map(|todo| todo.title.clone()))
}

#[update]
fn set_selected_todo_title(title: String) {
    selected_todo_title().set(title)
}

#[update]
fn save_selected_todo() {
    if let Some(title) = selected_todo_title().map_mut(Option::take) {
        if let Some(todo) = selected_todo().map_mut(Option::take) {
            todo.try_update(move |todo| todo.title = title);
        }
    }
}

// ------ Todos ------

#[derive(Deserialize, Serialize)]
struct Todo {
    id: TodoId,
    title: String,
    completed: bool,
}

#[model]
fn new_todo_title() -> String {
    String::new
}

#[update]
fn set_new_todo_title(title: String) {
    new_todo_title().set(title);
}

#[update]
fn create_todo(title: &str) {
    let title = title.trim();
    if title.is_empty() {
        return;
    }

    let mut todo = new_model(|| Todo {
        id: TodoId::new(),
        title,
        completed: false,
    });
    todo.on_change(store_todos);

    todos().update(|todos| todos.push(todo));
    new_todo_title().update(String::clear);
}

#[update]
fn remove_todo(todo: Model<Todo>) {
    if Some(todo) == selected_todo() {
        selected_todo().set(None);
    }
    todos().update(|todos| {
        if let Some(position) = todos.iter().position(|t| t == todo) {
            todos.remove(position);
        }
    });
    todo.try_remove();
}

#[update]
fn toggle_todo(todo: Model<Todo>) {
    todo.try_update(|todo| todo.checked = !todo.checked);
}

// -- all --

#[model]
fn todos() -> Vec<Todo> {
    LocalStorage::get(STORAGE_KEY).unwrap_or_default()
}

#[sub]
fn store_todos() {
    todos().use_ref(|todos| LocalStorage::insert(STORAGE_KEY, todos));
}

#[update]
fn check_or_uncheck_all(checked: bool) {
    if are_all_completed().inner() {
        todos().use_ref(|todos| todos.iter().for_each(toggle_todo));
    } else {
        active_todos().use_ref(|todos| todos.iter().for_each(toggle_todo));
    }
}

#[cache]
fn todos_count() -> usize {
    todos().map(Vec::len)
}

#[cache]
fn todos_exist() -> bool {
    todos_count().inner() != 0
}

// -- completed --

#[cache]
fn completed_todos() -> Vec<Todo> {
    todos().map(|todos| {
        todos
            .iter()
            .filter(|todo| todo.try_map(|todo| todo.completed).unwrap_or_default())
            .collect()
    })
}

#[update]
fn remove_completed() {
    completed_todos().use_ref(|todos| todos.iter().for_each(remove_todo));
}

#[cache]
fn completed_count() -> usize {
    completed_todos().map(Vec::len)
}

#[cache]
fn completed_exist() -> bool {
    completed_count().inner() != 0
}

#[cache]
fn are_all_completed() -> bool {
    todos_count().inner() == completed_count().inner()
}

// -- active --

#[cache]
fn active_todos() -> Vec<Todo> {
    todos().map(|todos| {
        todos
            .iter()
            .filter(|todo| todo.try_map(|todo| !todo.completed).unwrap_or_default())
            .collect()
    })
}

#[cache]
fn active_count() -> usize {
    active_todos().map(Vec::len)
}

// -- filtered --

#[cache]
fn filtered_todos() -> Cache<Vec<Todo>> {
    match app::selected_filter().inner() {
        Filter::All => app::todos().to_cache(),
        Filter::Active => app::active_todos(),
        Filter::Completed => app::completed_todos(),
    }
}
