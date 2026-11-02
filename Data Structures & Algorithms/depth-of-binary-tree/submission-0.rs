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

/* 
  int left_height = find_height_of_tree(node->left.get());
    int right_height = find_height_of_tree(node->right.get());

    return 1 + std::max(left_height, right_height);
*/

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node : &Option<Rc<RefCell<TreeNode>>>) -> i32{
            let Some(node) = node else {return 0 ;};
            
            let node = node.borrow();

            let left_count = dfs(&node.left);
            let right_count = dfs(&node.right);


            return 1 + left_count.max(right_count);
        }

        dfs(&root)
    }
}
