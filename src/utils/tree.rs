use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            ..Default::default()
        }
    }

    pub fn from_vec(items: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if items.is_empty() {
            return None;
        }

        let mut root = TreeNode::new(items[0]);
        root.fill(items, 0);
        Some(Rc::new(RefCell::new(root)))
    }

    pub fn fill(&mut self, items: &[i32], i: usize) {
        let li = i * 2 + 1;
        let ri = li + 1;

        if li >= items.len() {
            return;
        }

        let mut lb = TreeNode::new(items[li]);
        lb.fill(items, li);
        self.left = Some(Rc::new(RefCell::new(lb)));

        if ri >= items.len() {
            return;
        }

        let mut rb = TreeNode::new(items[ri]);
        rb.fill(items, ri);
        self.right = Some(Rc::new(RefCell::new(rb)));
    }

    pub fn to_sorted_vec(&self) -> Vec<i32> {
        let mut items = vec![];
        self.sorted_walk(&mut items);
        items
    }

    pub fn sorted_walk(&self, items: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().sorted_walk(items);
        }
        items.push(self.val);
        if let Some(right) = &self.right {
            right.borrow().sorted_walk(items);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sorted_vec() {
        let items = [4, 2, 7, 1, 3, 6, 9];
        let tree = TreeNode::from_vec(&items);
        assert!(tree.is_some());
        assert_eq!(
            tree.unwrap().borrow().to_sorted_vec(),
            vec![1, 2, 3, 4, 6, 7, 9]
        );
    }
}
