#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct TodoList {
    pub list: Vec<Todo>,
}

impl TodoList {
    pub fn add(&mut self, new_todo: String) {
        let new_todo = Todo {
            text: new_todo,
            completed: false,
        };

        self.list.push(new_todo);
    }

    pub fn remove_last_item(&mut self) {
        if self.list.is_empty() {
            println!("You cannot remove an item from an empty list.");
            return;
        }

        self.list.pop();
    }

    pub fn remove_all(&mut self) {
        self.list = Vec::new();
    }

    pub fn list_all_items(&self) {
        if self.list.is_empty() {
            println!("Your todo list is empty.");
            return;
        }

        for (index, todo) in self.list.iter().enumerate() {
            println!(
                "{} - {} is completed: {}",
                index + 1,
                todo.text,
                todo.completed
            );
        }
    }

    pub fn list_completed_items(&self) {
        if self.list.is_empty() {
            println!("Your todo list is empty.");
            return;
        }

        for (index, todo) in self.list.iter().enumerate() {
            if todo.completed {
                println!(
                    "{} - {} is completed: {}",
                    index + 1,
                    todo.text,
                    todo.completed
                );
            }
        }
    }

    pub fn list_uncompleted_items(&self) {
        if self.list.is_empty() {
            println!("Your todo list is empty.");
            return;
        }

        for (index, todo) in self.list.iter().enumerate() {
            if !todo.completed {
                println!(
                    "{} - {} is completed: {}",
                    index + 1,
                    todo.text,
                    todo.completed
                );
            }
        }
    }

    pub fn complete_item(&mut self, item_index: usize) {
        if item_index > self.list.len() || item_index <= 0 {
            println!("You can't edit a nonexistent item");
            return;
        }

        let todo = &mut self.list[item_index - 1];

        todo.completed = true;
    }
}
