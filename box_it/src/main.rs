// use std::mem;

// use box_it::*;

// fn main() {
//     let s = "5.5k 8.9k 32".to_owned();

//     let boxed = parse_into_boxed(s);
//     println!("Element value: {:?}", boxed[0]);
//     println!("Element size: {:?} bytes", mem::size_of_val(&boxed[0]));

//     let unboxed = into_unboxed(boxed);
//     println!("Element value: {:?}", unboxed[0]);
//     println!("Element size: {:?} bytes", mem::size_of_val(&unboxed[0]));

//     // As with everything related to regular Rust memory management, both the `Vec` and the `Box`es will be properly dropped when out of scope and freed, ensuring no leaks
// }
use std::{fmt::Debug, mem};

use box_it::*;

fn run_test<T: Debug + PartialEq>(s: &str, expected: T, predicate: impl FnOnce(String) -> T) {
    let new = predicate(s.to_owned());

    assert_eq!(new, expected);
    assert_eq!(mem::size_of_val(&new), mem::size_of::<T>());
}

#[test]
fn test_transform() {
    run_test(
        "5.5k 8.9k 32",
        vec![Box::new(5500), Box::new(8900), Box::new(32)],
        parse_into_boxed,
    );
    run_test(
        "6.8k 13.5k",
        vec![Box::new(6800), Box::new(13500)],
        parse_into_boxed,
    );
    run_test(
        "20.3k 3.8k 7.7k 992",
        vec![
            Box::new(20300),
            Box::new(3800),
            Box::new(7700),
            Box::new(992),
        ],
        parse_into_boxed,
    );
}

#[test]
fn test_take_value_from_box() {
    run_test("5.5k 8.9k 32", vec![5500, 8900, 32], |v| {
        into_unboxed(parse_into_boxed(v))
    });
    run_test("6.8k 13.5k", vec![6800, 13500], |v| {
        into_unboxed(parse_into_boxed(v))
    });
    run_test("20.3k 3.8k 7.7k 992", vec![20300, 3800, 7700, 992], |v| {
        into_unboxed(parse_into_boxed(v))
    });
}