use std::{collections::VecDeque, ops::{Index, IndexMut}};

#[derive(Clone)]
pub struct KeyedArray<T> {
    array: Vec<Option<T>>,
    free_locations: VecDeque<usize>,
}

impl<T: Clone> KeyedArray<T> {
    pub fn new(n: usize) -> Self {
        let mut a = Vec::<Option<T>>::new();
        a.resize(n, None);
        Self {
            array: a,
            free_locations: (0..n).collect(),
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        if let Some(next) = self.free_locations.pop_front() {
            self.array[next] = Some(value);
            next
        } else {
            self.array.push(Some(value));
            self.array.len() - 1
        }
    }

    pub fn remove(&mut self, id: usize) {
        if id >= self.array.len() { return }
        self.array[id] = None;
        self.free_locations.push_front(id);
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }
}

impl<T: Clone> Index<usize> for KeyedArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.array[index].as_ref().expect("Attempted to index unallocated ID")
    }
}

impl<T: Clone> IndexMut<usize> for KeyedArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.array[index].as_mut().expect("Attempted to index unallocated ID")
    }
}
