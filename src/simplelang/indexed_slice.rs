use std::ops::*;
use std::slice::Iter;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IndexedSlice<'a, T: 'a> {
    slice: &'a [T],
    position: usize,
}

impl<'a, T> IndexedSlice<'a, T> {
    pub fn from_slice(slice: &'a [T]) -> Self {
        IndexedSlice { slice, position: 0 }
    }

    pub fn iter(&self) -> Iter<T> {
        self.slice.iter()
    }

    pub fn pop_first(&mut self) -> Option<&T> {
        match self.len() {
            0 => None,
            _ => {
                let first = &self.slice[0];
                self.slice = &self.slice[1..];
                Some(first)
            }
        }
    }

    pub fn first(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            _ => Some(&self.slice[0]),
        }
    }

    pub fn pop_while<F>(&mut self, f: F) -> IndexedSlice<'a, T>
    where
        F: Fn(&T) -> bool,
    {
        let mut index = self.len();
        for i in 0..self.len() {
            if !f(&self[i]) {
                index = i;
                break;
            }
        }

        let result = IndexedSlice {
            position: self.position,
            slice: &self.slice[..index],
        };

        self.position += index;
        self.slice = &self.slice[index..];

        return result;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }
}

pub trait Get<I> {
    type Output;
    fn get(&self, index: I) -> Self::Output;
}

impl<'a, T> Get<Range<usize>> for IndexedSlice<'a, T> {
    type Output = IndexedSlice<'a, T>;

    fn get(&self, range: Range<usize>) -> Self::Output {
        do_slice(self, range.start, range)
    }
}

impl<'a, T> Get<RangeFrom<usize>> for IndexedSlice<'a, T> {
    type Output = IndexedSlice<'a, T>;

    fn get(&self, range: RangeFrom<usize>) -> Self::Output {
        do_slice(self, range.start, range)
    }
}

impl<'a, T> Get<RangeTo<usize>> for IndexedSlice<'a, T> {
    type Output = IndexedSlice<'a, T>;

    fn get(&self, range: RangeTo<usize>) -> Self::Output {
        do_slice(self, 0, range)
    }
}

impl<'a, T> Get<RangeFull> for IndexedSlice<'a, T> {
    type Output = IndexedSlice<'a, T>;

    fn get(&self, range: RangeFull) -> Self::Output {
        do_slice(self, 0, range)
    }
}

fn do_slice<'a, T, I>(idslice: &IndexedSlice<'a, T>, offset: usize, index: I) -> IndexedSlice<'a, T>
where
    [T]: Index<I, Output = [T]>,
{
    let position = idslice.position + offset;
    let slice = &idslice.slice[index];
    IndexedSlice { position, slice }
}

impl<'a, T> Index<usize> for IndexedSlice<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.slice[index]
    }
}

impl<'a, T: ToString> ToString for IndexedSlice<'a, T> {
    fn to_string(&self) -> String {

        let mut result = String::with_capacity(self.slice.len());
        for item in self.slice {
            let s = &item.to_string()[..];
            result.push_str(s);
        }
        return result;
    }
}
