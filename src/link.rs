use std::cell::RefCell;

pub struct Node<T> {
    data: T,
    next: Option<Box<Self>>,
}

impl<T> Node<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut T {
        &mut self.data
    }
}

#[derive(Default)]
pub struct NodeList<T> {
    head: Option<Node<T>>,
    idx: RefCell<usize>,
}

impl<T> NodeList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            idx: RefCell::new(0),
        }
    }

    pub fn push(&mut self, data: T) {
        let node = Node { data, next: None };

        if self.head.is_none() {
            self.head = Some(node);
            return;
        }

        let mut head = self.head.as_mut().unwrap();
        loop {
            head = match head.next {
                Some(ref mut next) => next,
                None => break,
            };
        }

        head.next = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut head = self.head.as_mut()?;

        loop {
            head = match head.next.as_mut() {
                Some(value) => value,
                _ => return None,
            };

            if head.next.is_some() && head.next.as_ref().unwrap().next.is_none() {
                // 倒数第二个结点
                let next = head.next.take();
                return next.map(|it| it.data);
            }
        }
    }

    pub fn unshift(&mut self, data: T) {
        self.head = Some(Node {
            data,
            next: self.head.take().map(Box::new),
        })
    }

    pub fn shift(&mut self) -> Option<T> {
        let ret = self.head.take();
        match ret {
            Some(Node { data, next }) => {
                self.head = next.map(|it| *it);
                Some(data)
            }
            None => None,
        }
    }

    pub fn insert(&mut self, data: T, idx: usize) -> Option<&Node<T>> {
        if idx == 0 {
            self.unshift(data);
            return self.head.as_ref();
        }

        let mut node = Node { data, next: None };

        let mut head = self.head.as_mut()?;
        for _ in 0..idx - 1 {
            head = head.next.as_mut()?;
        }

        node.next = head.next.take();
        head.next = Some(Box::new(node));

        head.next.as_deref()
    }
}

impl<T: std::cmp::PartialEq> NodeList<T> {
    pub fn find(&self, data: T) -> Option<&Node<T>> {
        let mut head = self.head.as_ref()?;

        loop {
            if head.data == data {
                return Some(head);
            }
            head = head.next.as_ref()?;
        }
    }
}

impl<T> std::ops::Index<usize> for NodeList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut head = self.head.as_ref();
        for _ in 0..index {
            head = head.unwrap().next.as_deref();
        }

        head.unwrap().data()
    }
}

impl<T> std::ops::IndexMut<usize> for NodeList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut head = self.head.as_mut();
        for _ in 0..index {
            head = head.unwrap().next.as_deref_mut();
        }

        head.unwrap().mut_data()
    }
}

impl<'a, T> Iterator for &'a NodeList<T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut idx = self.idx.borrow_mut();

        let mut head = self.head.as_ref()?;
        for _ in 0..*idx {
            head = match head.next.as_ref() {
                Some(next) => next,
                None => {
                    *idx = 0;
                    return None;
                }
            };
        }
        *idx += 1;

        Some(&head.data)
    }
}

pub struct Iter<'a, T> {
    head: Option<&'a Node<T>>,
}

impl<T> NodeList<T> {
    pub fn iter(&mut self) -> Iter<T> {
        Iter {
            head: self.head.as_ref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(head) => {
                self.head = head.next.as_deref();
                Some(&head.data)
            }
            None => None,
        }
    }
}

pub struct MutIter<'a, T> {
    head: Option<&'a mut Node<T>>,
}

impl<T> NodeList<T> {
    pub fn mut_iter(&mut self) -> MutIter<T> {
        MutIter {
            head: self.head.as_mut(),
        }
    }
}

impl<'a, T> Iterator for MutIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head.take() {
            Some(head) => {
                self.head = head.next.as_deref_mut();
                Some(&mut head.data)
            }
            None => None,
        }
    }
}
