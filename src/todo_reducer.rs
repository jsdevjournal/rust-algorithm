
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



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn reducer_1() {
        let mut state = TodoState { todos: vec![] };
        assert!(state.todos.is_empty());
        state = reducer(state, TodoAction::AddTodo(String::from("test")));
        assert_eq!(state.todos, vec!["test".to_string()]);
    }

    #[test]
    fn reducer_2() {
        let state = TodoState { todos: vec!["test".to_string()] };
        assert_eq!(state.todos, vec!["test".to_string()]);
    }

    #[test]
    fn reducer_3() {
        let mut state = TodoState { todos: vec!["test".to_string()] };
        state = reducer(state, TodoAction::RemoveTodo(0));
        assert!(state.todos.is_empty());
    }
}
