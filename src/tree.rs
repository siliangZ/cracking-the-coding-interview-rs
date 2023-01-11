use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

type Edge<T> = Rc<RefCell<TreeNode<T>>>;

pub struct TreeNodeWithParent {
    pub data: i32,
    pub left: Option<Rc<RefCell<TreeNodeWithParent>>>,
    pub right: Option<Rc<RefCell<TreeNodeWithParent>>>,
    pub parent: Option<Weak<RefCell<TreeNodeWithParent>>>,
}

/// a funciton to compare two option object with a cmp_method
fn compare_option<T, F>(op1: &Option<T>, op2: &Option<T>, cmp: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    if op1.is_none() && op2.is_none() {
        return true;
    }

    if let (Some(o1), Some(o2)) = (op1, op2) {
        cmp(o1, o2)
    } else {
        return false;
    }
}

impl PartialEq for TreeNodeWithParent {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.left == other.left
            && self.right == other.right
            && compare_option(&self.parent, &other.parent, |o1, o2| o1.ptr_eq(&o2))
    }
}

#[derive(PartialEq, Eq)]
pub struct TreeNode<T> {
    pub data: T,
    pub left: Option<Edge<T>>,
    pub right: Option<Edge<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    /// calculate the height of any tree that has node as root
    pub fn any_height(node: Rc<RefCell<TreeNode<T>>>) -> usize {
        let left_height = if let Some(left) = node.borrow().left.clone() {
            TreeNode::<T>::any_height(left)
        } else {
            0
        };
        let right_height = if let Some(right) = node.borrow().right.clone() {
            TreeNode::<T>::any_height(right)
        } else {
            0
        };

        std::cmp::max(left_height, right_height) + 1
    }
}

fn queue_to_string(queue: Vec<Option<Edge<&str>>>) -> String {
    let new: Vec<&str> = queue
        .into_iter()
        .map(|v| match v {
            Some(v) => v.borrow().data,
            None => "#",
        })
        .collect();
    new.join(",")
}

fn deserialize(root: Rc<RefCell<TreeNode<&str>>>) -> String {
    let mut queue = vec![Some(root)];
    let mut index = 0;
    while index < queue.len() {
        let node = queue.get(index).unwrap();
        index += 1;
        if let Some(n) = node {
            let new_left = n.borrow().left.as_ref().map(|r| r.clone());
            let new_right = n.borrow().right.as_ref().map(|r| r.clone());
            if new_left.is_none() && new_right.is_none() {
                continue;
            }
            queue.push(new_left);
            queue.push(new_right);
        }
    }

    queue_to_string(queue)
}

fn serialize(input: &'static str) -> Rc<RefCell<TreeNode<&str>>> {
    let data_arr: Vec<&str> = input.split(",").collect();
    let mut root = Rc::new(RefCell::new(TreeNode::new(data_arr[0])));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut add_to_left = true;
    let mut index = 0;
    for value in data_arr.into_iter().skip(1) {
        if value.ne("#") {
            let node = Rc::new(RefCell::new(TreeNode::new(value)));
            if add_to_left {
                queue[index].borrow_mut().left = Some(node.clone());
            } else {
                queue[index].borrow_mut().right = Some(node.clone());
            }
            queue.push_back(node);
        }

        if !add_to_left {
            index += 1;
        }
        add_to_left = !add_to_left;
    }
    return root;
}

#[cfg(test)]
mod tests {
    use super::{deserialize, serialize};

    #[test]
    fn test_deserialize() {
        let graph_str = "1,2,3,4,#,5,6";
        let root = serialize(graph_str);
        let convert = deserialize(root);
        assert_eq!(graph_str, &convert);
    }
}
