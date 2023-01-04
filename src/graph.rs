use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type GraphNode = Rc<RefCell<Node>>;
struct Node {
    data: &'static str,
    neightbors: Vec<GraphNode>,
}
