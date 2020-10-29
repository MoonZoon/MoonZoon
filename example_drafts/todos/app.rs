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
    zoon::model::url().map(Route::from)
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
    match route().inner() {
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
            title: todo.map(|todo| todo.title.clone()),
        });
    } else {
        selected_todo.set(None);
    }
}

#[Update]
fn set_selected_todo_title(title: String) {
    selected_todo().update(move |selected_todo| {
        if let Some(selected_todo) = selected_todo {
            selected_todo.title = title;
        }  
    });
}

#[Update]
fn save_selected_todo() {
    if let Some(selected_todo) = selected_todo().map_mut(Option::take) {
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
    new_todo_title().update(String::clear);
}

#[Update]
fn remove_todo(todo: Model<Todo>) {
    let todo_id = todo.map(|todo| todo.id);
    let selected_todo_id = selected_todo().map(|selected_todo| {
        selected_todo?.todo.map(|todo| Some(todo.id))
    });
    if let Some(selected_todo_id) = selected_todo_id {
        if selected_todo_id == todo_id {
            selected_todo().set(None);
        }
    }
    todos().update(|todos| todos.remove(todo_id));
    todo.remove();
}

#[Update]
fn toggle_todo(todo: Model<Todo>) {
    todo.update(|todo| todo.checked = !todo.checked);
}

// -- all --

#[Model]
fn todos() -> Todos {
    BTreeMap::new
}

#[Update]
fn check_or_uncheck_all(checked: bool) {
    if are_all_completed().inner() {
        todos().update_ref(|todos| todos.values().for_each(toggle_todo));
    } else {
        active_todos().update_ref(|todos| todos.values().for_each(toggle_todo));
    }
}

#[Cache]
fn todos_count() -> usize {
    todos().map(BTreeMap::len)
}

#[Cache]
fn todos_exist() -> bool {
    todos_count().inner() != 0
}

// -- completed --

#[Cache]
fn completed_todos() -> Todos {
    todos().map(|todos| {
        todos
            .iter()
            .filter(|_, todo| todo.completed)
            .collect()
    })
}

#[Update]
fn remove_completed() {
    completed_todos().update_ref(|todos| todos.values().for_each(remove_todo));
}

#[Cache]
fn completed_count() -> usize {
    completed_todos().map(BTreeMap::len)
}

#[Cache]
fn completed_exist() -> bool {
    completed_count().inner() != 0
}

#[Cache]
fn are_all_completed() -> bool {
    todos_count().inner() == completed_count().inner()
}

// -- active --

#[Cache]
fn active_todos() -> Todos {
    todos().map(|todos| {
        todos
            .iter()
            .filter(|_, todo| !todo.completed)
            .collect()
    })
}

#[Cache]
fn active_count() -> usize {
    active_todos().map(BTreeMap::len)
}

// -- filtered --

#[Cache]
fn filtered_todos() -> Todos {
    match selected_filter().inner() {
        Filter::All => todos(),
        Filter::Active => active_todos(),
        Filter::Completed => completed_todos(),
    }
}
