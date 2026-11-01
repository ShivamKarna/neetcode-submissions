// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(node : &Option<Rc<RefCell<TreeNode>>>,min : i64, max : i64)-> bool{
            let Some(n) = node else {return true};

            let n = n.borrow();
            let val = n.val as i64;

            if val <= min || val >= max {
                return false;
            }
            // if part crossed so now val is valid

            validate(&n.left,min,val) && validate(&n.right,val,max)
        }
        validate(&root,i64::MIN,i64::MAX)
    }
}
