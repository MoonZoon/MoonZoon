use std::sync::Arc;
use zoon::{eprintln, println, strum::EnumIter, *};

pub mod view;

mod todo_id;
use todo_id::TodoId;

static STORAGE_KEY: &str = "todomvc-zoon";

// ------ ------
//     Types
// ------ ------

// ------ Filter -------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, Deserialize, Serialize)]
#[serde(crate = "serde")]
#[strum(crate = "strum")]
pub enum Filter {
    All,
    Active,
    Completed,
}

// ------ Todo -------

#[derive(Deserialize, Serialize)]
#[serde(crate = "serde")]
struct Todo {
    id: TodoId,
    title: Mutable<String>,
    completed: Mutable<bool>,
    #[serde(skip)]
    edited_title: Mutable<Option<String>>,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn selected_filter() -> &'static Mutable<Filter> {
    Mutable::new(Filter::All)
}

#[static_ref]
fn todos() -> &'static MutableVec<Arc<Todo>> {
    MutableVec::new()
}

#[static_ref]
fn selected_todo() -> &'static Mutable<Option<Arc<Todo>>> {
    Mutable::new(None)
}

#[static_ref]
fn new_todo_title() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//    Caches
// ------ ------

#[static_ref]
fn are_all_completed() -> &'static ReadOnlyMutable<bool> {
    let mutable = Mutable::new(false);
    let read_only = mutable.read_only();
    Task::start(
        all_and_completed()
            .map(|(all, completed)| all == completed)
            .for_each_sync(move |all_completed| {
                mutable.set_neq(all_completed);
            }),
    );
    read_only
}

// ------ ------
//   Signals
// ------ ------

fn todos_count() -> impl Signal<Item = usize> {
    todos().signal_vec_cloned().len()
}

fn todos_exist() -> impl Signal<Item = bool> {
    todos_count().map(|count| count != 0).dedupe()
}

fn completed_count() -> impl Signal<Item = usize> {
    todos()
        .signal_vec_cloned()
        .map_signal(|todo| todo.completed.signal().dedupe())
        .filter(|completed| *completed)
        .len()
}

fn completed_exist() -> impl Signal<Item = bool> {
    completed_count().map(|count| count != 0).dedupe()
}

fn all_and_completed() -> impl Signal<Item = (usize, usize)> {
    map_ref! {
        let all = todos_count(),
        let completed = completed_count() =>
        (*all, *completed)
    }
}

fn active_count() -> impl Signal<Item = usize> {
    all_and_completed()
        .map(|(all, completed)| all - completed)
        .dedupe()
}

fn filtered_todos() -> impl SignalVec<Item = Arc<Todo>> {
    todos()
        .signal_vec_cloned()
        .map_signal(|todo| todo.completed.signal().dedupe().map(move |_| todo.clone()))
        .filter_signal_cloned(|todo| {
            let completed = todo.completed.get();
            selected_filter()
                .signal()
                .dedupe()
                .map(move |filter| match filter {
                    Filter::All => true,
                    Filter::Active => not(completed),
                    Filter::Completed => completed,
                })
        })
}

fn is_todo_selected(id: TodoId) -> impl Signal<Item = bool> {
    selected_todo()
        .signal_ref(move |todo| {
            if let Some(todo) = todo {
                todo.id == id
            } else {
                false
            }
        })
        .dedupe()
}

fn is_filter_selected(filter: Filter) -> impl Signal<Item = bool> {
    selected_filter()
        .signal()
        .map(move |selected_filter| selected_filter == filter)
        .dedupe()
}

// ------ ------
//   Commands
// ------ ------

pub fn load_todos() {
    if let Some(Ok(todos)) = local_storage().get(STORAGE_KEY) {
        replace_todos(todos);
        println!("Todos loaded");
    }
}

fn save_todos() {
    if let Err(error) = local_storage().insert(STORAGE_KEY, todos()) {
        eprintln!("Saving todos failed: {:?}", error);
    }
}

fn replace_todos(new_todos: Vec<Arc<Todo>>) {
    todos().update_mut(|todos| {
        todos.clear();
        todos.extend(new_todos);
    });
}

pub fn select_filter(filter: Filter) {
    selected_filter().set_neq(filter);
}

fn select_todo(todo: Option<Arc<Todo>>) {
    if let Some(todo) = &todo {
        if todo.edited_title.map(|title| title.is_none()) {
            todo.edited_title.set(Some(todo.title.get_cloned()))
        }
    }
    selected_todo().set(todo);
}

fn set_new_todo_title(title: String) {
    new_todo_title().set(title)
}

fn save_selected_todo() {
    let todo = selected_todo().take().unwrap_throw();
    let new_title = todo.edited_title.take().unwrap_throw();
    todo.title.set(new_title);
    save_todos();
}

fn add_todo() {
    let mut new_todo_title = new_todo_title().lock_mut();
    let title = new_todo_title.trim();
    if title.is_empty() {
        return;
    }
    let todo = Todo {
        id: TodoId::new(),
        title: Mutable::new(title.to_owned()),
        completed: Mutable::new(false),
        edited_title: Mutable::new(None),
    };
    todos().lock_mut().push_cloned(Arc::new(todo));
    save_todos();
    new_todo_title.clear();
}

fn set_todo_completed(todo: &Todo, completed: bool) {
    todo.completed.set_neq(completed);
    save_todos();
}

fn remove_todo(id: TodoId) {
    todos().lock_mut().retain(|todo| todo.id != id);
    save_todos();
}

fn remove_completed_todos() {
    todos().lock_mut().retain(|todo| not(todo.completed.get()));
    save_todos();
}

fn check_or_uncheck_all_todos() {
    let completed = are_all_completed().get();
    for todo in todos().lock_ref().iter() {
        todo.completed.set_neq(!completed);
    }
    save_todos();
}
