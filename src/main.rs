mod todo_list;
fn main() {
    let mut todo = todo_list::TodoList { list: Vec::new() };

    todo.add(String::from("todo 1"));
    todo.add(String::from("todo 2"));

    todo.remove_last_item();
    todo.remove_last_item();

    todo.add(String::from("todo 1"));
    todo.add(String::from("todo 2"));

    todo.remove_all();

    todo.add(String::from("todo 1"));
    todo.add(String::from("todo 2"));

    todo.complete_item(2);

    todo.add(String::from("todo 3"));

    todo.complete_item(3);

    println!("\nAll Items: ");
    todo.list_all_items();

    println!("\n-----------------------------------------------\n");

    println!("Completed Items: ");
    todo.list_completed_items();

    println!("\n-----------------------------------------------\n");

    println!("Uncompleted Items: ");
    todo.list_uncompleted_items();

    println!("\n-----------------------------------------------\n");

    println!("{:?}", todo);
}
