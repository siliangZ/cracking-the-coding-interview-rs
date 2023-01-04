use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type Edge = Rc<RefCell<TreeNode>>;
struct TreeNode {
    data: &'static str,
    left: Option<Edge>,
    right: Option<Edge>,
}

impl TreeNode {
    pub fn new(data: &'static str) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}

fn queue_to_string(queue: Vec<Option<Edge>>) -> String {
    let new: Vec<&str> = queue
        .into_iter()
        .map(|v| match v {
            Some(v) => v.borrow().data,
            None => "#",
        })
        .collect();
    new.join(",")
}

fn deserialize(root: Rc<RefCell<TreeNode>>) -> String {
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

fn serialize(input: &'static str) -> Rc<RefCell<TreeNode>> {
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
