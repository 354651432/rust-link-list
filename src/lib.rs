#![allow(unused)]

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Display,
    mem::{self, take},
};

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Self>>,
}

pub struct Link<T> {
    pub head: Option<Node<T>>,
    idx: RefCell<usize>,
}

impl<T: std::cmp::PartialEq> Link<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            idx: RefCell::new(0),
        }
    }

    pub fn make_test() -> Link<u32> {
        Link {
            head: Some(Node::<u32> {
                data: 10,
                next: Some(Box::new(Node::<u32> {
                    data: 20,
                    next: Some(Box::new(Node::<u32> {
                        data: 99,
                        next: None,
                    })),
                })),
            }),
            idx: RefCell::new(0),
        }
    }

    pub fn push(&mut self, item: T) {
        let node = Node::<T> {
            data: item,
            next: None,
        };

        let mut pt = self.head.as_mut();
        match pt {
            Some(mut tail) => {
                while tail.next.is_some() {
                    tail = &mut *tail.next.as_mut().unwrap();
                }

                tail.next = Some(Box::new(node));
            }
            None => self.head = Some(node),
        }
    }

    pub fn tail(&self) -> Option<&T> {
        let mut tail = self.head.as_ref();
        loop {
            let mut next = &tail.unwrap().next;
            if next.is_none() {
                break;
            }
            tail = Some(&next.as_ref().unwrap());
        }
        Some(&tail?.data)
    }

    pub fn my_map(&mut self, f: impl Fn(&T) -> T) {
        let ret = self.tail();

        let mut tail = self.head.as_mut();
        loop {
            if tail.is_none() {
                break;
            }

            let inner = tail.unwrap();

            inner.data = f(&inner.data);

            tail = inner.next.as_deref_mut();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut tail = self.head.as_mut();
        loop {
            let mut inner = tail?;

            if inner.next.is_some() && inner.next.as_ref().unwrap().next.is_none() {
                let last = inner.next.take();
                return last.map(|it| it.data);
            }

            tail = inner.next.as_deref_mut();
        }
    }

    pub fn find(&self, item: T) -> Option<&Node<T>> {
        let mut head = self.head.as_ref();
        loop {
            if head?.data == item {
                return head;
            }
            head = head?.next.as_ref().map(|it| &**it)
        }
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        let mut node = Node::<T> {
            data: item,
            next: None,
        };

        let mut pt = self.head.as_mut();
        if idx <= 0 {
            node.next = self.head.take().map(|it| Box::new(it));
            self.head = Some(node);
            return;
        }

        match pt {
            Some(mut tail) => {
                for _ in 0..idx - 1 {
                    tail = match tail.next.as_mut() {
                        Some(next) => next,
                        None => return,
                    };
                }
                node.next = tail.next.take();
                tail.next = Some(Box::new(node));
            }
            None => self.head = Some(node),
        }
    }

    pub fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(ref mut p) = head {
            // println!("drop {}", p.data);
            head = p.next.take().map(|node| *node);
        }
    }
}

impl<'a, T> Iterator for &'a Link<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut p = match &self.head {
            Some(p) => p,
            None => return None,
        };
        let mut idx = self.idx.borrow_mut();
        for _ in 0..*idx {
            p = match p.next {
                Some(ref next) => next,
                None => {
                    *idx = 0;
                    return None;
                }
            };
        }
        *idx += 1;
        Some(&p.data)
    }
}

impl<T> Iterator for Link<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let head = self.head.take();

        match head {
            Some(head) => {
                let data = head.data;
                self.head = head.next.map(|it| *it);
                Some(data)
            }
            None => None,
        }
    }
}
