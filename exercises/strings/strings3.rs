// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


use std::fmt::format;

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string() // trim()除了单词间的空格全部消除，包括首尾的空格,返回一个字符串切片,所以需要to_string()转换为String
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    // 第二个方法：format!宏
    // format!("{} world!", input)
    // 第三个方法：String::from()
    // String::from(input) + " world!"
    // 第四个方法：push_str()方法
    // let mut s = String::from(input);
    // s.push_str(" world!");
    // s
    // 第五个方法：push()方法
    // let mut s = String::from(input);
    // s.push(' ');
    // s.push_str("world!");
    // s

    input.to_string() + " world!"
    // 两个字符串相加，需要转换为String,或者使用format!宏,或者使用String::from(),或者使用push_str()方法,或者使用push()方法
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // 第一个方法：replace()方法
    input.replace("cars", "balloons")
    // 第二个方法：replace_range()方法
    // let mut s = String::from(input);
    // s.replace_range(7..11, "balloons");
    // s
    // 第三个方法：replace()方法
    // let mut s = String::from(input);
    // s.replace("cars", "balloons")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
