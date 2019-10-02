use crate::linked_list_items::Node;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
        }
    }

    pub fn len(&self) -> usize {
        match &self.head {
            Some(node) => node.len(),
            None => 0,
        }
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            Some(node) => {
                let data = node.data;
                self.head = node.next;
                Some(data)
            },
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None,
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = SimpleLinkedList::new();
        let mut node = &self.head;
        while let Some(n) = node {
            list.push(n.data.clone());
            node = &n.next;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in item {
            list.push(i.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut node = self.head;
        let mut vec = Vec::new();
        while let Some(n) = node {
            vec.insert(0, n.data);
            node = n.next;
        }
        vec
    }
}

pub mod linked_list_items {
    #[derive(Debug)]
    pub struct Node<T> {
        pub data: T,
        pub next: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
            Node {
                data,
                next,
            }
        }

        pub fn len(&self) -> usize {
            match &self.next {
                Some(node) => 1 + node.len(),
                None => 1,
            }
        }
    }
}
