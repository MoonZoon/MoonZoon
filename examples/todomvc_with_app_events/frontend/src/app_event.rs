use crate::{Route, Todo};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct NewTodoTitleChanged {
    pub title: String,
}

#[derive(Clone, Copy)]
pub(crate) struct ToggleAllCheckboxClicked;

#[derive(Clone)]
pub(crate) struct TodoCheckboxChanged {
    pub todo: Todo,
    pub checked: bool,
}

#[derive(Clone)]
pub(crate) struct TodoTitleDoubleClicked {
    pub todo: Todo,
    pub title: Arc<String>,
}

#[derive(Clone)]
pub(crate) struct RemoveTodoButtonPressed {
    pub todo: Todo,
}

#[derive(Clone)]
pub(crate) struct EditingTodoTitleBlurredOrEnterPressed {
    pub todo: Todo,
    pub edited_title: Arc<String>,
}

#[derive(Clone)]
pub(crate) struct EditingTodoTitleChanged {
    pub todo: Todo,
    pub text: String,
}

#[derive(Clone, Copy)]
pub(crate) struct EditingTodoTitleEscapePressed;

#[derive(Clone, Copy)]
pub(crate) struct FilterPressed {
    pub route: Route,
}

#[derive(Clone, Copy)]
pub(crate) struct ClearCompletedButtonPressed;

#[derive(Clone, Copy)]
pub(crate) struct RouteChanged {
    pub route: Option<Route>,
}

#[derive(Clone)]
pub(crate) struct NewTodoTitlePreparedForSaving {
    pub title: Arc<String>,
}
