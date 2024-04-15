/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
// 定义了二叉树节点，每个二叉树有Option类型的左右叶子节点
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>, // 每个裸指针包裹在一个Option类型的枚举里面，然后是一个智能指针，指向二叉树上的一个点
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
// 定义了二叉树，指定跟节点
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>, // 是一个指向根节点的智能指针
}

// 二叉树节点初始化
impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

// 二叉树节点的方法
impl<T> TreeNode<T>
where
    T: Ord,
{
    // 此时self是二叉树里面的一个节点
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 要插入的节点小于当前节点
            Ordering::Less => {
                // 应该把它往左边递归
                match &mut self.left {
                    Some(left) => {
                        // 如果左指针有值，就继续递归
                        // 左指针是一个Box
                        (*left).insert(value);
                    }
                    _ => {
                        // 如果左指针指向None，就让它指向新建节点
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            Ordering::Greater => {
                // 应该把它往右边递归
                match &mut self.right {
                    Some(right) => {
                        // 如果右指针有值，就继续递归
                        // 右指针是一个Box
                        (*right).insert(value);
                    }
                    _ => {
                        // 如果右指针指向None，就让它指向新建节点
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            _ => {} // 相等时不插入新节点
        }
    }

    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 说明应该往当前节点的左边找
                match &self.left {
                    Some(left) => { return left.search(value); }
                    _ => { return false; }
                }
            }
            Ordering::Greater => {
                // 说明应该往当前节点的右边找
                match &self.right {
                    Some(right) => { return right.search(value); }
                    _ => { return false; }
                }
            }
            _ => { return true; }
        }
    }
}

// 二叉树的方法
impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 直接使用节点的插入方法
        match &mut self.root {
            Some(root_node) => {
                root_node.insert(value);
            }
            _ => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match &self.root {
            Some(root_node) => { root_node.search(value) }
            _ => { false }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


