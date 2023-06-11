use std::{cell::RefCell, rc::Rc};

use crate::utils::TreeNode;

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert(&root)
}

pub fn invert(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let Some(node) = node else {
        return None;
    };
    let left = invert(&node.borrow().right);
    let right = invert(&node.borrow().left);
    Some(Rc::new(RefCell::new(TreeNode {
        val: node.borrow().val,
        left,
        right,
    })))
}

#[cfg(test)]
mod test {
    use crate::utils::TreeNode;

    use super::invert_tree;

    #[test]
    fn check_multiple() {
        let tree = TreeNode::from_vec(&[4, 2, 7, 1, 3, 6, 9]);
        assert_eq!(
            invert_tree(tree).unwrap().borrow().to_sorted_vec(),
            vec![9, 7, 6, 4, 3, 2, 1]
        );
    }

    fn check_three() {
        let tree = TreeNode::from_vec(&vec![2, 1, 3]);
        let inverted_tree = invert_tree(tree).unwrap().borrow().to_sorted_vec();

        assert_eq!(inverted_tree, vec![3, 2, 1]);
    }

    #[test]
    fn check_none() {
        let tree = TreeNode::from_vec(&vec![]);
        let inverted_tree = invert_tree(tree);

        assert_eq!(inverted_tree, None);
    }
}
