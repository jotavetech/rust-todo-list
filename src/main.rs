mod todo_list;
fn main() {
    let mut todo = todo_list::TodoList { list: Vec::new() };

    todo.add(String::from("todo 1"));
    todo.add(String::from("todo 2"));

    todo.remove_last_item();

    todo.add(String::from("todo 2"));
    todo.add(String::from("todo 3"));

    todo.remove_all();

    todo.remove_last_item();

    todo.add(String::from("todo 1"));
    todo.add(String::from("todo 2"));

    todo.list_items();

    println!("{:?}", todo);
}
