use std::{cell::RefCell, rc::Rc};

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
        let node = Rc::new(RefCell::new(TreeNode::new(
            array[start].to_string().as_str(),
        )));
        return Some(node);
    }

    let mid = (start + end) / 2;
    let root = Rc::new(RefCell::new(TreeNode::new(array[mid])));

    let left_node = divide_and_conquer(start, mid - 1, &array);

    let right_node = divide_and_conquer(mid + 1, end, &array);

    root.borrow_mut().left = left_node;
    root.borrow_mut().right = right_node;

    Some(root)
}

// implement the function to traverse the bst
fn inorder_traversal(root: Rc<RefCell<TreeNode<u32>>>) -> Vec<u32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::minimal_tree;

    #[test]
    fn test_dc() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let root = minimal_tree(nums);
    }
}
