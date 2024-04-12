// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

enum MyStr<T> {
    Str(T)
}

fn main() {
    let mut shopping_list: Vec<MyStr<String>> = Vec::new();
    shopping_list.push(MyStr::<String>::Str("milk".to_string()));
}
