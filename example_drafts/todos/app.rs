use zoon::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ulid::Ulid;


type TodoId = Ulid;
type Todos = BTreeMap<TodoId, Model<Todo>>;

// ------ Routes ------

#[Route]
#[derive(Copy, Clone)]
enum Route {
    #[path("active")]
    Active,
    #[path("completed")]
    Completed,
    #[path()]
    Root,
    Unknown,
}

#[Cache]
fn route() -> Route {
    let url = zoon::model::url();

    url.map(Route::from)
}

#[Update]
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

#[Model]
fn filters() -> Vec<Filter> {
    Filter::iter().collect()
}

#[Cache]
fn selected_filter() -> Filter {
    let route = selected_route();

    match route.inner() {
        Route::Active => Filter::Active,
        Route::Completed => Filter::Completed,
        _ => Filter::All,
    }
}

// ------ SelectedTodo ------

struct SelectedTodo {
    todo: Model<Todo>,
    title: String,
}

#[Model]
fn selected_todo() -> Option<SelectedTodo> {
    None
}

#[Update]
fn select_todo(todo: Option<Model<Todo>>) {
    if Some(todo) = todo {
        selected_todo.set(SelectedTodo {
            todo,
            title: todo.map(|t| t.title.clone()),
        });
    } else {
        selected_todo.set(None);
    }
}

#[Update]
fn set_selected_todo_title(title: String) {
    selected_todo().update(move |todo| todo.title = title);
}

#[Update]
fn save_selected_todo() {
    if let Some(selected_todo) = selected_todo().take_inner() {
        let todo = selected_todo.todo;
        todo.update(|todo| todo.title = selected_todo.title);
    }
}

// ------ Todos ------

struct Todo {
    id: TodoId,
    title: String,
    completed: bool,
}

#[Model]
fn new_todo_title() -> String {
    String::new
}

#[Update]
fn set_new_todo_title(title: String) {
    new_todo_title().set(title);
}

#[Update]
fn create_todo(title: &str) {
    let title = title.trim();
    if title.is_empty() {
        return;
    }

    let todo_id = TodoId::new();
    let mut todo = new_model(|| Todo {
        id: todo_id,
        title: title.trim(),
        completed: false,
    });

    todos().update(|todos| todos.insert(todo_id, todo));
    new_todo_title().update(|title| title.clear());
}

#[Update]
fn remove_todo(todo: Model<Todo>) {
    let Some(selected_todo_id) = selected_todo().map(|t| t.map(|t| t.id)) {
        if selected_todo_id == todo.map(|t| t.id) {
            selected_todo().set(None);
        }
    }
    todos().update(|todos| todos.remove(todo.inner().id));
    todo.remove();
}

#[Update]
fn toggle_todo(todo: Model<Todo>) {
    todo.update(|todo| *todo.checked = !todo.checked);
}

// -- all --

#[Model]
fn todos() -> Todos {
    BTreeMap::new
}

#[Update]
fn check_or_uncheck_all(checked: bool) {
    if are_all_completed().inner() {
        todos().update(|todos| {
            for todo in todos.values() {
                toggle_todo(todo)
            }
        })
    } else {
        active_todos().update(|todos| {
            for todo in todos.values() {
                toggle_todo(todo)
            }
        })
    }
}

#[Cache]
fn todos_count() -> usize {
    let todos = todos();

    todos.map(BTreeMap::len)
}

#[Cache]
fn todos_exist() -> bool {
    let todos_count = todos_count();

    todos_count().inner() != 0
}

// -- completed --

#[Cache]
fn completed_todos() -> Todos {
    let todos = todos();

    todos.map(|todos| {
        todos
            .iter()
            .filter(|_, todo| todo.completed)
            .collect()
    })
}

#[Update]
fn remove_completed() {
    for todo in completed_todos().inner().values() {
        remove_todo(todo);
    }
}

#[Cache]
fn completed_count() -> usize {
    let todos = completed_todos();

    todos.map(BTreeMap::len)
}

#[Cache]
fn completed_exist() -> bool {
    let todos_count = completed_count();

    todos_count().inner() != 0
}

#[Cache]
fn are_all_completed() -> bool {
    let total_count = todos_count();
    let completed_count = completed_count();

    total_count.inner() == completed_count.inner()
}

// -- active --

#[Cache]
fn active_todos() -> Todos {
    let todos = todos();

    todos.map(|todos| {
        todos
            .iter()
            .filter(|_, todo| !todo.completed)
            .collect()
    })
}

#[Cache]
fn active_count() -> usize {
    let todos = active_todos();

    todos.map(BTreeMap::len)
}

// -- filtered --

#[Cache]
fn filtered_todos() -> Todos {
    let selected_filter = selected_filter();

    match selected_filter.inner() {
        Filter::All => todos(),
        Filter::Active => active_todos(),
        Filter::Completed => completed_todos(),
    }
}
