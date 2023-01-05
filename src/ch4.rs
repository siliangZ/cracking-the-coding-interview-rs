use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use super::tree::TreeNode;
fn minimal_tree(array: Vec<u32>) -> Option<Rc<RefCell<TreeNode<u32>>>> {
    divide_and_conquer(0, array.len() - 1, &array)
}

fn divide_and_conquer(
    start: usize,
    end: usize,
    array: &Vec<u32>,
) -> Option<Rc<RefCell<TreeNode<u32>>>> {
    if start > end {
        return None;
    }

    if start == end {
        let node = Rc::new(RefCell::new(TreeNode::new(array[start])));
        return Some(node);
    }

    let mid = (start + end) / 2;
    let root = Rc::new(RefCell::new(TreeNode::new(array[mid])));

    let left_node = if mid == 0 {
        None
    } else {
        divide_and_conquer(start, mid - 1, &array)
    };

    let right_node = divide_and_conquer(mid + 1, end, &array);

    root.borrow_mut().left = left_node;
    root.borrow_mut().right = right_node;

    Some(root)
}

// recursion implementation the function to traverse the bst
fn inorder_traversal_recursion(root: Rc<RefCell<TreeNode<u32>>>, result: &mut Vec<u32>) {
    if let Some(left) = root.borrow().left.as_ref() {
        inorder_traversal_recursion(left.clone(), result)
    }

    result.push(root.borrow().data);

    if let Some(right) = root.borrow().right.as_ref() {
        inorder_traversal_recursion(right.clone(), result);
    }
}

fn level_traversal(root: Rc<RefCell<TreeNode<u32>>>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back(root);
    let mut index = 0;
    while index < queue.len() {
        let node = queue[index].clone();
        index += 1;
        if let Some(left) = node.borrow().left.clone() {
            queue.push_back(left)
        }

        let right_borrow = node.borrow();
        if let Some(right) = right_borrow.right.clone() {
            queue.push_back(right)
        }
    }

    result
}

fn inorder_traversal(root: Rc<RefCell<TreeNode<u32>>>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut stack = Vec::new();

    let mut node = Some(root.clone());
    while let Some(inner) = node {
        node = inner.borrow().left.as_ref().map(|r| r.clone());
        stack.push(inner);
    }

    while let Some(node) = stack.last().cloned() {
        result.push(node.borrow().data);
        if let Some(right) = node.borrow().right.clone() {
            let mut node = Some(right);
            while let Some(inner) = node {
                node = inner.borrow().left.clone();
                stack.push(inner)
            }
        } else {
            while let Some(node) = stack.pop() {
                if let Some(last) = stack.last().cloned() {
                    if last.borrow().left == Some(node) {
                        break;
                    }
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{ch4::inorder_traversal, tree::TreeNode};

    use super::{inorder_traversal_recursion, minimal_tree};

    #[test]
    fn test_dc() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let root: Rc<RefCell<TreeNode<u32>>> = minimal_tree(nums).unwrap();
        //println!("tree root: {:?}", root.borrow().data);
        //if let Some(left) = root.borrow().left.as_ref() {
        //println!("left: {:?}", left.borrow().data);
        //};
        //if let Some(left) = root.borrow().right.as_ref() {
        //println!("right: {:?}", left.borrow().data);
        //};
        let mut recursion_result = Vec::new();
        inorder_traversal_recursion(root.clone(), &mut recursion_result);

        let result = inorder_traversal(root);

        println!("{:?}", result);
    }
}
