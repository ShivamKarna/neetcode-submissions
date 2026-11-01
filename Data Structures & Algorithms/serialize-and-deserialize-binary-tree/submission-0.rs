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

use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(node : &Option<Rc<RefCell<TreeNode>>>,bytes : &mut Vec<u8>){
            match node{
                None => bytes.push(0), // tag 0 saying there is no node
                Some(n) =>{
                    let n = n.borrow();

                    bytes.push(1); // tag 1 for telling there is node
                    bytes.extend_from_slice(&n.val.to_le_bytes());

                    dfs(&n.left,bytes);
                    dfs(&n.right,bytes);
                }
            }
        }

        let mut bytes = Vec::<u8>::new();
        dfs(&root,&mut bytes);
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {

        let bytes : Vec<u8> = data
            .as_bytes()
            .chunks_exact(2)
            .map(|c| {
                u8::from_str_radix(std::str::from_utf8(c).unwrap(),16).unwrap()   // first , str from utf8, then u8 from str
            })
            .collect();

        fn dfs(bytes: &[u8],i: &mut usize)->Option<Rc<RefCell<TreeNode>>>{
            let marker = bytes[*i];
            *i += 1;

            if marker == 0{
                return None;
            }

            let val = i32::from_le_bytes(bytes[*i..*i+4].try_into().unwrap());

            *i += 4;

            let left = dfs(bytes,i);
            let right = dfs(bytes, i);

            Some(Rc::new(RefCell::new(TreeNode{val,left,right})))
        }
        dfs(&bytes,&mut 0)
    }
}
