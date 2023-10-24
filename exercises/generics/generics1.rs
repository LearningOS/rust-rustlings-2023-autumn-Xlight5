// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // 占用符：使编译器自己进行推断类型，而不是使用泛型，这样就不会报错。
    let mut shopping_list: Vec<_> = Vec::new();
    shopping_list.push("milk");
}
