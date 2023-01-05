//! The data structure copied from leetcode

use std::{cell::RefCell, rc::Rc};

pub struct TreeNodeLC {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNodeLC>>>,
    pub right: Option<Rc<RefCell<TreeNodeLC>>>,
}

pub struct ListNodeLC {
    pub val: i32,
    pub next: Option<Box<ListNodeLC>>,
}
