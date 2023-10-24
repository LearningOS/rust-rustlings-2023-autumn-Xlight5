// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map.values().filter(|&&val| val == value).count() // 这里的val是&&Progress类型，所以需要解引用两次
    // 第二种方法
    // iter()获取map的迭代器
// filter()HashMap中第一个元素value相等的值
// count()用来返回数量
    map.iter().filter(|v| *v.1 == value).count()
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().map(|map| count_iterator(map, value)).sum()
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

// fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
//     // map is a hashmap with String keys and Progress values.
//     // map = { "variables1": Complete, "from_str": None, ... }
//     todo!();
// }

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // let mut count = 0;
    // for map in collection {
    //     for val in map.values() {
    //         if val == &value {
    //             count += 1;
    //         }
    //     }
    // }
    // count
    // 第二种方法

    let mut my_map1 = collection[0].clone();
    let mut my_map2 = collection[1].clone();
    // 调用extend方法，将my_map1转换为一个迭代器，并将其键值对添加到my_map2中
    my_map2.extend(my_map1.into_iter());
    // 与上一个相同
    // let count11 = my_map1.iter().filter(|v| *v.1 == value).count();
    // let count33 = my_map1.iter().filter(|v| *v.1 == Progress::Some).count();
    // let count44 = my_map1.iter().filter(|v| *v.1 == Progress::None).count();

    let count22 = my_map2.iter().filter(|v| v.1 == &value).count();
    // let count55 = my_map2.iter().filter(|v| *v.1 == Progress::Some).count();
    // let count66 = my_map2.iter().filter(|v| *v.1 == Progress::None).count();

     count22

}

// fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
//     // collection is a slice of hashmaps.
//     // collection = [{ "variables1": Complete, "from_str": None, ... },
//     //     { "variables2": Complete, ... }, ... ]
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state)
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
