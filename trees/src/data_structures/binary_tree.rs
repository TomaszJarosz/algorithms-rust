#[derive(Debug)]
pub struct BinaryTree {
    pub val: i32,
    pub left: Option<Box<BinaryTree>>,
    pub right: Option<Box<BinaryTree>>,
}


impl BinaryTree {
    pub fn new(value: i32) -> Self {
        Self {
            val: value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        if value < self.val {
            match &mut self.left {
                Some(left_child) => left_child.insert(value),
                None => self.left = Some(Box::new(BinaryTree::new(value)))
            }
        } else {
            match &mut self.right {
                Some(right_child) => right_child.insert(value),
                None => self.right = Some(Box::new(BinaryTree::new(value)))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::data_structures::binary_tree::BinaryTree;

    #[test]
    fn test_new_tree() {
        let tree = BinaryTree::new(10);
        assert_eq!(tree.val, 10);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());
    }

    #[test]
    fn test_insert_left() {
        let mut tree = BinaryTree::new(10);
        tree.insert(3);
        assert_eq!(tree.val, 10);
        assert_eq!(tree.left.as_ref().unwrap().val, 3);
        assert!(tree.right.is_none());
    }

    #[test]
    fn test_insert_right() {
        let mut tree = BinaryTree::new(10);
        tree.insert(15);
        assert!(tree.right.is_some());
        assert_eq!(tree.right.as_ref().unwrap().val, 15);
        assert!(tree.left.is_none());
    }

    #[test]
    fn test_multiple_inserts() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(20);

        assert_eq!(tree.val, 10);
        assert_eq!(tree.left.as_ref().unwrap().val, 5);
        assert_eq!(tree.left.as_ref().unwrap().left.as_ref().unwrap().val, 3);
        assert_eq!(tree.left.as_ref().unwrap().right.as_ref().unwrap().val, 7);
        assert_eq!(tree.right.as_ref().unwrap().val, 15);
        assert_eq!(tree.right.as_ref().unwrap().left.as_ref().unwrap().val, 12);
        assert_eq!(tree.right.as_ref().unwrap().right.as_ref().unwrap().val, 20);
    }

    #[test]
    fn test_in_order_traversal() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);

        // Zbieramy wynik in-order traversal do wektora
        let mut result = Vec::new();
        fn collect_in_order(tree: &BinaryTree, result: &mut Vec<i32>) {
            if let Some(left) = &tree.left {
                collect_in_order(left, result);
            }
            result.push(tree.val);
            if let Some(right) = &tree.right {
                collect_in_order(right, result);
            }
        }
        collect_in_order(&tree, &mut result);

        // Sprawdzamy, czy wynik jest posortowany
        assert_eq!(result, vec![3, 5, 7, 10, 15]);
    }
}