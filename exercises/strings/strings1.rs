// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // let a = "blue";
    let mut a = "444".to_string();
    let b = &mut a;
    let c = &mut a;


    let answer = current_favorite_color(c);
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color(s : &mut  String) -> String { // 返回字符串切片，但是因为切片是引用类型，离开这个函数就失效了，所以需要加上'static

    s.push_str("red");
    // s = "red";
    s.to_string()
}




    // 第二种方法：将&str转换为String，toString（），然后返回String，这样就不需要'static了，
    // 但是这样会多一次内存分配，所以不推荐，但是如果是返回一个变量的话，就可以这样做，因为变量是在堆上分配的，所以不会出现问题，
    // 但是字符串字面量是在栈上分配的，所以会出现问题。

