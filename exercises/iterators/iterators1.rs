// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.


#[test]
fn main() {
    let fruits = vec!["banana", "peach",];
    let mut iter_fruits = fruits.into_iter();

    assert_eq!(iter_fruits.next(), Some("banana"));
    assert_eq!(iter_fruits.next(), Some("peach"));    
    assert_eq!(iter_fruits.next(), None);
}
