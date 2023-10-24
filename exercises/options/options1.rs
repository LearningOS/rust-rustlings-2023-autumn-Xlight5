// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day > 24 {
        None
    } else if time_of_day >= 22 {
        Some(0)
    } else {
        Some(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);

        // 使用unwrap()方法，获取Option中的值，如果是None，则会panic，程序会崩溃，所以要注意使用，可以使用match来处理，也可以使用unwrap_or()方法，如果是None，则返回默认值，不会panic，也可以使用unwrap_or_else()方法，如果是None，则执行闭包中的代码，不会panic，也可以使用expect()方法，如果是None，则会panic，但是可以自定义panic的信息，也可以使用unwrap_err()方法，如果是Some，则会panic，但是可以自定义panic的信息。
        println!("{}", icecreams.unwrap());
        assert_eq!(icecreams.unwrap(), 5);
    }

    // #[test]
    // fn main() {
    //     let icecreams = maybe_icecream(12);
    //     println!("{}", icecreams.unwrap());
    // }
}

fn main() {
    let icecreams = maybe_icecream(12);
    println!("哈哈哈哈哈哈哈哈哈哈{}", icecreams.unwrap());
}
