// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a i32, y: i32) ->  &'a i32{
    if *x > y {
        x
    } else {
        x
    }
}

fn main() {
    let string1 = 123123;
    let string2 = 123123123;


    let result = longest(&string1, string2);
    println!("The longest string is '{}'", result);
    println!("{}",string2.to_string())
}

