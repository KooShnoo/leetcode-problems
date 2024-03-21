use std::cell::RefCell;
use std::rc::Rc;
/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let Some(root) = root else {
        return None;
    };
    let mut unsearched = vec![root];
    while !unsearched.is_empty() {
        let Some(node) = unsearched.pop() else{
            return None;
        };
        if node.borrow().val == val {
            return Some(node)
        }
        if node.borrow().left.is_some() {
            unsearched.push(node.borrow_mut().left.clone().unwrap())
        }
        if node.borrow().right.is_some() {
            unsearched.push(node.borrow_mut().right.clone().unwrap())
        }
    }
    None
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};
    use super::*;

    #[test]
    fn t1() {
        let mut root = TreeNode::new(4);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.left.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let val = 2;
        assert!(search_bst(Some(Rc::new(RefCell::new(root))), val).is_some())

        // Output: [2,1,3]
    }

    #[test]
    fn t2() {
        let mut root = TreeNode::new(4);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.left.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let val = 5;
        assert!(search_bst(Some(Rc::new(RefCell::new(root))), val).is_none())

        // Output: []
        
    }
}
