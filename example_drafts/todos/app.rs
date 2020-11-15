use zoon::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ulid::Ulid;

mod els;

const STORAGE_KEY: &str = "todos-zoon";

type TodoId = Ulid;

blocks!{
    append_blocks![els]

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
        url().map(Route::from)
    }

    #[update]
    fn set_route(route: Route) {
        url().set(Url::from(route))
    }

    // ------ Filters ------

    #[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
    enum Filter {
        All,
        Active,
        Completed,
    }

    #[var]
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

    #[var]
    fn selected_todo() -> Option<Var<Todo>> {
        None
    }

    #[update]
    fn select_todo(todo: Option<Var<Todo>>) {
        selected_todo().set(todo)
    }

    #[var]
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
                todo.try_update_mut(move |todo| todo.title = title);
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

    #[var]
    fn todo_event_handler() -> VarEventHandler<Todo> {
        VarEventHandler::new(|event, todo| match event {
            VarEvent::Create => todos().update_mut(|todos| todos.push(todo)),
            VarEvent::Change => todos().mark_updated(),
            VarEvent::Remove => {
                todos().update_mut(|todos| {
                    if let Some(position) = todos.iter().position(|t| t == todo) {
                        todos.remove(position);
                    }
                });
            }
        })
    }

    #[var]
    fn new_todo_title() -> String {
        String::new()
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

        let mut todo = Var::new(Todo {
            id: TodoId::new(),
            title,
            completed: false,
        });
        new_todo_title().update_mut(String::clear);
    }

    #[update]
    fn remove_todo(todo: Var<Todo>) {
        if Some(todo) == selected_todo() {
            selected_todo().set(None);
        }
        todo.try_remove();
    }

    #[update]
    fn toggle_todo(todo: Var<Todo>) {
        todo.try_update_mut(|todo| todo.checked = !todo.checked);
    }

    // -- all --

    #[var]
    fn todos() -> Vec<Var<Todo>> {
        LocalStorage::get(STORAGE_KEY).unwrap_or_default()
    }

    #[subscription]
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
    fn completed_todos() -> Vec<Var<Todo>> {
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
    fn active_todos() -> Vec<Var<Todo>> {
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
    fn filtered_todos() -> Cache<Vec<Var<Todo>>> {
        match selected_filter().inner() {
            Filter::All => todos().to_cache(),
            Filter::Active => active_todos(),
            Filter::Completed => completed_todos(),
        }
    }

}
