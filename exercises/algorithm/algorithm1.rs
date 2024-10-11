/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// 1I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;
use std::clone::Clone;
#[derive(Debug)]
struct Node<T:Clone+ std::cmp::PartialOrd> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T:Clone+ std::cmp::PartialOrd> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T:Clone+ std::cmp::PartialOrd> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T:Clone+ std::cmp::PartialOrd> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T:Clone+ std::cmp::PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(mut list_a: LinkedList<T>,mut list_b:LinkedList<T>) -> Self
	{
		//TODO
		let mut list_new = Self {
            length: 0,
            start: None,
            end: None,
        };
        list_new.length = list_a.length + list_b.length;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        for _ in 0..list_new.length{
            let ele_a;
            let ele_b;
            if let Some(p) = list_a.get(i) {
                ele_a = Some(p.clone());
            }else{
                ele_a = None;
            }
            if let Some(p) = list_b.get(j) {
                ele_b = Some(p.clone());
            }else{
                ele_b = None;
            }
            match (ele_a, ele_b) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        list_new.add(a);
                        i += 1; 
                    } else {
                        list_new.add(b);
                        j += 1;
                    }
                }
                (Some(a), None) => {
                    list_new.add(a);
                    i += 1;
                }
                (None, Some(b)) => {
                    list_new.add(b);
                    j += 1;
                }
                _ => ()
            }
        }
        list_new
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display+Clone+ std::cmp::PartialOrd,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display+Clone+ std::cmp::PartialOrd,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}