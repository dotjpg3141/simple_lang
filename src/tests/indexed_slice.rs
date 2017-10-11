use std::ops::*;
use std::slice::Iter;
use simplelang::indexed_slice::*;

#[test]
fn basic_slice() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());
    assert_slice(slice, "ABCDE", 0, 5);

}

#[test]
fn slice_range() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let current = slice.get(1..3);
    assert_slice(current, "BC", 1, 2);
    assert_slice(current.get(1..2), "C", 2, 1);
}

#[test]
fn slice_range_from() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let current = slice.get(3..);
    assert_slice(current, "DE", 3, 2);
    assert_slice(current.get(1..), "E", 4, 1);
}

#[test]
fn slice_range_to() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let current = slice.get(..2);
    assert_slice(current, "AB", 0, 2);
    assert_slice(current.get(..1), "A", 0, 1);
}

#[test]
fn slice_range_full() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let current = slice.get(..);
    assert_slice(current, "ABCDE", 0, 5);
    assert_slice(current.get(..), "ABCDE", 0, 5);
}

#[test]
fn slice_pop_first() {

    let text: Vec<char> = "AB".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let mut current = slice;
    assert_eq!(current.pop_first(), Some(&'A'));
    assert_eq!(current.pop_first(), Some(&'B'));
    assert_eq!(current.pop_first(), None);
}

#[test]
fn slice_first() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    assert_eq!(slice.first(), Some(&'A'));
}

#[test]
fn slice_pop_while() {

    let text: Vec<char> = "ABCDE".chars().collect();
    let slice = IndexedSlice::from_slice(text.as_slice());

    let mut current = slice.clone();
    assert_slice(current.pop_while(|c| c <= &'C'), "ABC", 0, 3);
    assert_slice(current, "DE", 3, 2);
}

fn assert_slice(slice: IndexedSlice<char>, text: &str, position: usize, len: usize) {
    assert_eq!(slice.to_string(), text);
    assert_eq!(slice.position(), position);
    assert_eq!(slice.len(), len);
}
