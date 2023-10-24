// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<'a> {
    value:  &'a i32,
}

impl <'a> Wrapper<'a> {
    pub fn new(value: &'a i32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn store_u32_in_wrapper() {
    //     assert_eq!(Wrapper::new(42).value, 42);
    // }

    #[test]
    fn store_str_in_wrapper() {
        // let a = "Foo";
        let  b = 123;
        let c = & b;
        assert_eq!(Wrapper::new(c).value, &123);
    }
}
