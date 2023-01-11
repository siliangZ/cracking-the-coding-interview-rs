use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::tree::TreeNodeWithParent;

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

fn check_balanced(root: Rc<RefCell<TreeNode<u32>>>) -> bool {
    let (left_height, is_left_balanced) = root
        .borrow()
        .left
        .clone()
        .map(|node| (TreeNode::any_height(node.clone()), check_balanced(node)))
        .unwrap_or((0, true));
    let (right_height, is_right_balanced) = root
        .borrow()
        .right
        .clone()
        .map(|node| (TreeNode::any_height(node.clone()), check_balanced(node)))
        .unwrap_or((0, true));

    if is_left_balanced && is_right_balanced && right_height.abs_diff(left_height) <= 1 {
        return true;
    }
    false
}

// Q4.5 inorder traversal the BST and check if the result is in non-descending order

fn find_successor_bst(
    node: Rc<RefCell<TreeNodeWithParent>>,
) -> Option<Rc<RefCell<TreeNodeWithParent>>> {
    if let Some(mut root) = node.borrow().right.clone() {
        // moved outside the while let expression to shorten the lifetime of Rec<'_,_> returned by borrow()
        let mut r_n = root.borrow().left.clone();
        while let Some(current) = r_n {
            root = current;
            r_n = root.borrow().left.clone();
        }
        Some(root)
    } else {
        // if the node doesn't have, retrace to its parent node to find the fist one that comes to the node from left branch
        let mut current = node.clone();
        let mut parent = current.borrow().parent.clone().and_then(|n| n.upgrade());
        while let Some(inner) = parent.clone() {
            if inner.eq(&current) {
                break;
            }
            current = inner;
            parent = current.borrow().parent.clone().and_then(|n| n.upgrade());
        }
        parent
    }
}

fn create_adjacent_map_and_indegree_dict(
    projects: Vec<char>,
    dependencies: Vec<(char, char)>,
) -> (HashMap<char, usize>, HashMap<char, Vec<char>>) {
    let mut indegree_dict = HashMap::new();
    let mut adjacent_map = HashMap::new();

    for (require, dependent) in dependencies.iter() {
        indegree_dict
            .entry(*dependent)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        adjacent_map
            .entry(*require)
            .and_modify(|v: &mut Vec<char>| v.push(*dependent))
            .or_insert(vec![*dependent]);
    }
    (indegree_dict, adjacent_map)
}
// it is toplogical sorting
fn build_order(projects: Vec<char>, dependencies: Vec<(char, char)>) -> Option<Vec<char>> {
    let (mut indegree_dict, adjacent_map) =
        create_adjacent_map_and_indegree_dict(projects, dependencies);
    // find all zero indegree char and insert into a vector
    let mut candidates: Vec<char> = indegree_dict
        .iter()
        .filter_map(|(k, v)| if *v == 0 { Some(*k) } else { None })
        .collect();

    let mut index = 0;
    while index < candidates.len() {
        let extend = candidates.get(index).and_then(|c| {
            adjacent_map.get(c).map(|r| {
                let mut extend = Vec::new();
                for c in r {
                    if let Some(degree) = indegree_dict.get_mut(c) {
                        *degree -= 1;
                        if *degree == 0 {
                            extend.push(*c);
                        }
                    }
                }
                extend
            })
        });
        if let Some(mut extend) = extend {
            candidates.append(&mut extend);
        }
        index += 1;
    }

    if candidates.is_empty() {
        None
    } else {
        Some(candidates)
    }
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
