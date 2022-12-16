//! it comes from the book https://rust-unofficial.github.io/too-many-lists/first-layout.html
// issues
// 1. one of the nodes is not heap-allocated
// 2. allocate a node that is actually not a node
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
pub enum ListBad<T> {
    Cons(T, Box<ListBad<T>>),
    Nil,
}

// avoid the extra junk
// uniformly allocate
// null pointer optimization
// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, elem: T) {
        let node = Node {
            elem,
            // we remove the add it back later
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                Some((*node).elem)
            }
            Link::Empty => None,
        }
    }
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}
