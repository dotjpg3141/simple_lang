use std::ops::*;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IndexedSlice<'a, T: 'a> {
    slice: &'a [T],
    position: usize,
}

impl<'a, T> IndexedSlice<'a, T> {
    pub fn from_slice(slice: &'a [T]) -> Self {
        IndexedSlice { slice, position: 0 }
    }

    pub fn pop_first(&mut self) -> Option<&'a T> {
        match self.len() {
            0 => None,
            _ => {
                let first = &self.slice[0];
                self.slice = &self.slice[1..];
                self.position += 1;
                Some(first)
            }
        }
    }

    pub fn first(self) -> Option<&'a T> {
        match self.len() {
            0 => None,
            _ => Some(&self.slice[0]),
        }
    }

    pub fn try_get(self, index: usize) -> Option<&'a T> {
        match self.len() {
            0 | 1 => None,
            _ => Some(&self.slice[index]),
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }
}

impl<'a> IndexedSlice<'a, char> {
    pub fn from_chars<V>(list: &'a V) -> IndexedSlice<'a, char>
    where
        V: 'a + Index<RangeFull, Output = [char]>,
    {
        let slice = &list[..];
        IndexedSlice { slice, position: 0 }
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
