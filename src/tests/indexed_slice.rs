use std::ops::*;
use std::slice::Iter;
use simplelang::indexed_slice::*;

fn char_vec(text: &str) -> Vec<char> {
    return text.chars().collect();
}

#[test]
fn basic_slice() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_chars(&text);

    assert_slice(slice, "ABCDE", 0, 5);
}

#[test]
fn slice_range() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_slice(text.as_slice());

    let slice = slice.get(1..3);
    assert_slice(slice, "BC", 1, 2);
    assert_slice(slice.get(1..2), "C", 2, 1);
}

#[test]
fn slice_range_from() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_slice(text.as_slice());

    let slice = slice.get(3..);
    assert_slice(slice, "DE", 3, 2);
    assert_slice(slice.get(1..), "E", 4, 1);
}

#[test]
fn slice_range_to() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_slice(text.as_slice());

    let slice = slice.get(..2);
    assert_slice(slice, "AB", 0, 2);
    assert_slice(slice.get(..1), "A", 0, 1);
}

#[test]
fn slice_range_full() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_slice(text.as_slice());

    let slice = slice.get(..);
    assert_slice(slice, "ABCDE", 0, 5);
    assert_slice(slice.get(..), "ABCDE", 0, 5);
}

#[test]
fn slice_pop_first() {

    let text = char_vec("AB");
    let mut slice = IndexedSlice::from_slice(text.as_slice());

    assert_slice(slice, "AB", 0, 2);
    assert_eq!(slice.pop_first(), Some(&'A'));
    assert_slice(slice, "B", 1, 1);
    assert_eq!(slice.pop_first(), Some(&'B'));
    assert_slice(slice, "", 2, 0);
    assert_eq!(slice.pop_first(), None);
    assert_slice(slice, "", 2, 0);
}

#[test]
fn slice_first() {

    let text = char_vec("ABCDE");
    let slice = IndexedSlice::from_slice(text.as_slice());

    assert_eq!(slice.first(), Some(&'A'));
}

fn assert_slice(slice: IndexedSlice<char>, text: &str, position: usize, len: usize) {
    assert_eq!(slice.to_string(), text);
    assert_eq!(slice.position(), position);
    assert_eq!(slice.len(), len);
}
