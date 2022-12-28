use std::collections::HashSet;

pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // map takes self by value
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

fn remove_duplicates<T: PartialEq, Eq>(list: &mut List<T>) {
    let mut buf = Vec::new();
    while let Some(current) = list.pop() {
        if buf.contains(&current) {
            continue;
        }
        buf.push(current)
    }
    // inplace reverse
    buf.reverse();

    for elem in buf {
        list.push(elem)
    }
}
