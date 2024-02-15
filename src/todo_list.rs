#[derive(Debug)]
pub struct TodoList {
    pub list: Vec<String>,
}

impl TodoList {
    pub fn add(&mut self, new_todo: String) {
        self.list.push(new_todo);
    }

    pub fn remove_last_item(&mut self) {
        if self.list.is_empty() {
            println!("You cannot remove an item from a empty list.");
            return;
        }

        self.list.pop();
    }

    pub fn remove_all(&mut self) {
        self.list = Vec::new();
    }

    pub fn list_items(&self) {
        if self.list.is_empty() {
            println!("Your todo list is empty.");
            return;
        }

        for (index, todo) in self.list.iter().enumerate() {
            println!("{} - {}", index + 1, todo);
        }
    }
}
