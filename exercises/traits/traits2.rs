// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar{
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // let mut v = self;
        self.push(String::from("Bar"));
        self
    }
}


#[cfg(test)]
mod tests {
    // use std::clone;
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut a = vec![String::from("Foo")];
        let mut foo = AppendBar::append_bar(a.clone());
        let mut bar = a.append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
