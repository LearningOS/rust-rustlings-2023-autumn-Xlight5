// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.


trait AppendBar {
    fn append_bar123(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar123(self) -> Self {
        format!("{}{}", self, "Bar")
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar123();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar123(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar123().append_bar123(),
            String::from("BarBar")
        );
    }
}
