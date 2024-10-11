/*
	heap
	This question requires you to implement a binary heap function
*/
// 1I AM NOT DONE

use std::cmp::Ord;
use std::default::Default;
#[derive(Debug)]
pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        let  vec = &mut self.items;
        vec.push(value);
        self.count += 1;
        let mut idx = self.count;
        while idx > 1 {
            let pi = idx / 2;
            if (self.comparator)(&self.items[idx], &self.items[pi]) {
                self.items.swap(idx, pi);
                idx = pi;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let l = idx*2;
        let r = idx*2+1;
        if l > self.count {
            return 0;
        }
        if r <= self.count{
            if (self.comparator)(&self.items[r], &self.items[l]){
                return r;
            }else{
                return l;
            }
        }
        return l;
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty(){
            return None;
        }
        let re = self.items.swap_remove(1);
        self.count -= 1;
        let mut idx = 1;
        while self.children_present(idx){
            let si = self.smallest_child_idx(idx);
            if idx == 0 {break}
            let sv = self.items.get(si).unwrap();
            let pv = self.items.get(idx).unwrap();
            if !(self.comparator)(pv,sv){
                self.items.swap(si,idx);
                idx = si;
            }else{
                break;
            }
        }
        return Some(re);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        println!("{:?}",heap);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        println!("{:?}",heap);
        assert_eq!(heap.next(), Some(4));
        println!("{:?}",heap);
        assert_eq!(heap.next(), Some(9));
        println!("{:?}",heap);
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}