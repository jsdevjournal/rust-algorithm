
pub enum TodoAction {
    AddTodo(String),
    RemoveTodo(usize),
}

#[derive(Debug)]
pub struct TodoState {
    pub todos: Vec<String>,
}

pub fn reducer(mut state: TodoState, action: TodoAction) -> TodoState {
    match action {
        TodoAction::AddTodo(todo) => {
            state.todos.push(todo);
        },
        TodoAction::RemoveTodo(index) => {
            state.todos.remove(index);
        },
    }
    state
}
