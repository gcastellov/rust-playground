use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();

        while let Some(head) = current {
            count = count + 1;
            current = head.next.as_ref();
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        let next: Option<Box<Node<T>>> = self.head.take();
        let new_node = Some(Box::new(Node { data: _element, next: next }));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(current) = self.head.take() {
            self.head = current.next;
            return Some(current.data);
        }

        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(current) = &self.head {
            return Some(&current.data);
        }

        None
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut values: Vec<T> = Vec::default();
        
        while let Some(current) = self.pop() {
            values.push(current);
        }

        for value in values {
            list.push(value);
        }

        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut out: Vec<T> = Vec::default();

        if let Some(node) = self.head {
            let mut current: Node<T> = *node;
            out.push(current.data);

            while current.next.is_some() {
                current = *current.next.unwrap();
                out.push(current.data);
            }

            out.reverse();
        }

        out
    }
}
