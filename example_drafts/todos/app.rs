type TodoId = Ulid;
type Todos = BTreeMap<TodoId, Model<Todo>>;

// ------ Filters ------

// derive Iter / to_vec
enum Filter {
    All,
    Completed,
    Active,
}

#[Model]
fn filters() -> Model<Vec<Filter>> {
    use_model(|| Filter::iter().collect())
}

#[Cache]
fn selected_filter() -> Cache<Filter> {
    let url = watch(zoon::model::url());

    use_cache(url.changed(), || {
        url.map(Filter::from)
    })
}

// ------ SelectedTodo ------

struct SelectedTodo {
    id: TodoId,
    title: String,
}

#[Model]
fn selected_todo() -> Model<SelectedTodo> {
    use_model(|| None)
}

// ------ Todos ------

struct Todo {
    id: TodoId,
    title: String,
    completed: bool,
}

#[Model]
fn new_todo_title() -> Model<String> {
    use_model(String::new)
}

#[Update]
fn new_todo(title: String) -> Model<Todo> {
    let todo_id = TodoId::new();

    let mut todo = new_model(|| Todo {
        id: todo_id,
        title,
        completed: false,
    });
    todo.on_remove(|todo| todos().update(|todos| todos.remove(todo.id)));
    
    todos().update(|todos| todos.insert(todo_id, todo));

    todo
}

// -- all --

#[Model]
fn todos() -> Model<Todos> {
    use_model(BTreeMap::new)
}

#[Cache]
fn todos_count() -> Cache<usize> {
    let todos = watch(todos());

    use_cache(todos.changed(), || {
        todos.map(BTreeMap::len)
    })
}

#[Cache]
fn todos_exist() -> Cache<bool> {
    let todos_count = watch(todos_count());

    use_cache(todos_count.changed(), || {
        todos_count().get() != 0
    })
}

// -- completed --

#[Cache]
fn completed_todos() -> Cache<Todos> {
    let todos = watch(todos);

    use_cache(todos.changed(), || {
        todos.map(|todos| ...)
    })
}

#[Cache]
fn completed_count() -> Cache<usize> {
    let todos = watch(completed_todos());

    use_cache(completed_count.changed(), || {
        completed_count.map(BTreeMap::len)
    })
}

#[Cache]
fn completed_exist() -> Cache<bool> {
    let todos_count = watch(completed_count());

    use_cache(todos_count.changed(), || {
        todos_count().get() != 0
    })
}

#[Cache]
fn are_all_completed() -> Cache<bool> {
    let total_count = watch(todos_count());
    let completed_count = watch(completed_count());

    use_cache(total_count.changed() || completed_count.changed(), || {
        total_count.get() == completed_count.get()
    })
}

// -- active --

#[Cache]
fn active_todos() -> Cache<Todos> {
    let todos = watch(todos());

    use_cache(todos.changed(), || {
        todos.map(|todos| ...)
    })
}

#[Cache]
fn active_count() -> Cache<usize> {
    let todos = watch(active_todos());

    use_cache(todos.changed(), || {
        todos.map(BTreeMap::len)
    })
}

// -- filtered --

#[Cache]
fn filtered_todos() -> Cache<Todos> {
    let selected_filter = watch(selected_filter());

    use_cache(selected_filter.changed(), || {
        match selected_filter.get() {
            Filter::All => todos(),
            Filter::Active => active_todos(),
            Filter::Completed => completed_todos(),
        }
    })
}
