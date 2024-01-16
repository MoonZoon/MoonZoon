use crate::{Route, Todo};

#[derive(Clone)]
pub struct NewTodoTitleChanged {
    pub title: String,
}

#[derive(Clone, Copy)]
pub struct ToggleAllCheckboxClicked;

#[derive(Clone)]
pub struct TodoCheckboxChanged {
    pub todo: Todo,
    pub checked: bool,
}

#[derive(Clone)]
pub struct TodoTitleDoubleClicked {
    pub todo: Todo,
}

#[derive(Clone)]
pub struct RemoveTodoButtonPressed {
    pub todo: Todo,
}

#[derive(Clone, Copy)]
pub struct EditingTodoTitleBlurredOrEnterPressed;

#[derive(Clone)]
pub struct EditingTodoTitleChanged {
    pub todo: Todo,
    pub text: String,
}

#[derive(Clone, Copy)]
pub struct EditingTodoTitleEscapePressed;

#[derive(Clone, Copy)]
pub struct FilterPressed {
    pub route: Route,
}

#[derive(Clone, Copy)]
pub struct ClearCompletedButtonPressed;

#[derive(Clone, Copy)]
pub struct RouteChanged {
    pub route: Option<Route>,
}

#[derive(Clone)]
pub struct SelectedTodoToSaveTaken {
    pub todo: Todo,
}

#[derive(Clone)]
pub struct EditedTitleToSaveTaken {
    pub todo: Todo,
    pub title: String,
}

#[derive(Clone)]
pub struct NewTodoTitleReadyToSave {
    pub title: String,
}
