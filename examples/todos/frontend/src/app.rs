use zoon::*;
// use serde::{Deserialize, Serialize};
use strum::EnumIter;
use uuid::Uuid;
use std::sync::Arc;
use std::ops::Deref;

pub mod view;

const STORAGE_KEY: &str = "todos-zoon";

// ------ ------
//     Types
// ------ ------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Filter {
    All,
    Active,
    Completed,
}

// #[derive(Deserialize, Serialize)]
struct Todo {
    id: TodoId,
    title: Mutable<String>,
    completed: Mutable<bool>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct TodoId(Uuid);

impl Deref for TodoId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ------ ------
//    Statics
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
fn selected_todo() -> &'static Mutable<Option<TodoId>> {
    Mutable::new(None)
}

#[static_ref]
fn selected_todo_title() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn new_todo_title() -> &'static Mutable<String> {
    Mutable::new(String::new())
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

fn todos_with_completed_signal() -> impl SignalVec<Item = Arc<Todo>> {
    todos()
        .signal_vec_cloned()
        .map_signal(|todo| todo.completed.signal().map(move |_| todo.clone()))
}

fn completed_todos() -> impl SignalVec<Item = Arc<Todo>> {
    todos_with_completed_signal()
        .filter(|todo| todo.completed.get())
}

fn completed_count() -> impl Signal<Item = usize> {
    completed_todos().len()
}      

fn completed_exist() -> impl Signal<Item = bool> {
    completed_count().map(|count| count != 0).dedupe()
}

fn are_all_completed() -> impl Signal<Item = bool> {
    (map_ref! {
        let todos_count = todos_count(),
        let completed_count = completed_count() =>
        todos_count == completed_count
    }).dedupe()
}

fn active_todos() -> impl SignalVec<Item = Arc<Todo>> {
    todos_with_completed_signal()
        .filter(|todo| not(todo.completed.get()))
}

fn active_count() -> impl Signal<Item = usize> {
    active_todos().len()
}    

fn filtered_todos() -> impl SignalVec<Item = Arc<Todo>> {
    todos_with_completed_signal().filter_signal_cloned(|todo| {
        let completed = todo.completed.get();
        selected_filter().signal().map(move |filter| match filter {
            Filter::All => true,
            Filter::Active => not(completed),
            Filter::Completed => completed,
        })
    })
}

fn is_todo_selected(id: TodoId) -> impl Signal<Item = bool> {
    selected_todo().signal().map(move |selected_id| {
        selected_id.map(|selected_id| selected_id == id).unwrap_or_default()
    }).dedupe()
}

fn is_filter_selected(filter: Filter) -> impl Signal<Item = bool> {
    selected_filter().signal().map(move |selected_filter| {
        selected_filter == filter
    }).dedupe()
}

// ------ ------
//   Commands
// ------ ------

pub fn select_filter(filter: Filter) {
    selected_filter().set_neq(filter);
}

fn select_todo(id: Option<TodoId>) {
    selected_todo().set_neq(id);
}

fn set_selected_todo_title(title: String) {
    selected_todo_title().set(Some(title))
}

fn set_new_todo_title(title: String) {
    new_todo_title().set(title)
}

fn save_selected_todo() {

}

fn add_todo() {
    let mut new_todo_title = new_todo_title().lock_mut();
    let title = new_todo_title.trim();
    if title.is_empty() {
        return;
    }
    let todo = Todo {
        id: TodoId(Uuid::new_v4()),
        title: Mutable::new(title.to_owned()),
        completed: Mutable::new(false),
    };
    todos().lock_mut().push_cloned(Arc::new(todo));
    new_todo_title.clear();
}

fn remove_todo(id: TodoId) {
    todos().lock_mut().retain(|todo| todo.id != id);
}

fn toggle_todo(todo: &Todo) {
    todo.completed.update(|completed| !completed);
}

fn remove_completed_todos() {
    todos().lock_mut().retain(|todo| not(todo.completed.get()));
}

fn set_all_todos_completed(completed: bool) {
    for todo in todos().lock_ref().iter() {
        todo.completed.set_neq(completed);
    }
}
