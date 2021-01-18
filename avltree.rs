#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::max;
use std::cmp::Ordering;

type Nodeptr<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    value: T,
    left: Nodeptr<T>,
    right: Nodeptr<T>,
    height: usize,   
}

#[derive(Debug, PartialEq, Clone)]
pub struct AVLTree<T> {
    root: Nodeptr<T>,
}

impl<T: Ord + Copy> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            left: None,
            right: None,
            height: 1,
        }
    }
}

impl<T: Ord + Copy> AVLTree<T> {
    pub fn new() -> AVLTree<T> {
        AVLTree { 
            root: None 
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root.take() {
            Some(root) => { self.root = self.insert_helper_fuction_1(Some(root), value) },
            None => { self.root = self.new_node(value); },
        }
    }

    fn insert_helper_fuction_1(&mut self, root: Nodeptr<T>, value: T) -> Nodeptr<T> {
        if let Some(mut root) = root {
            match root.value.cmp(&value) {            
                Ordering::Greater => root.left = self.insert_helper_fuction_2(root.left, value),
                Ordering::Less => root.right = self.insert_helper_fuction_2(root.right, value),
                Ordering::Equal => root.value = value,
            }
            self.update_height(&mut root);
            self.rebalance(Some(root))
        }
        else { None }
    }

    fn insert_helper_fuction_2(&mut self, root: Nodeptr<T>, value: T) -> Nodeptr<T> {
        match root {
            Some(root) => self.insert_helper_fuction_1(Some(root), value),
            None => self.new_node(value),
        }
    }

    fn rebalance(&mut self, root: Nodeptr<T>) -> Nodeptr<T> { // if not balanced then rebalance based on balance factor (BF > 1 -> Rebalance Left & BF < -1 -> Rebalance Right)
        let bf = self.balance_factor(&root);
        match bf {
            -1 | 0 | 1 => { root },
            _ => {
                if self.balance_factor(&root) > 1 {
                    self.rebalance_left_side(root)            
                } 
                else {
                    self.rebalance_right_side(root)                   
                } 
            }
        }  
    }

    fn rebalance_left_side(&mut self, root: Nodeptr<T>) -> Nodeptr<T> {
        if let Some(mut root) = root {
            let left_subtree = root.left.take();
            if let Some(left_subtree) = left_subtree {
                if self.get_height(&left_subtree.left) > self.get_height(&left_subtree.right) {
                    root.left = Some(left_subtree);                    
                } else { 
                    root.left = self.left_rotation(Some(left_subtree));
                 }
                self.right_rotation(Some(root))
            } else { None }
        } else { None }
    }

    fn rebalance_right_side(&mut self, root: Nodeptr<T>) -> Nodeptr<T> {
        if let Some(mut root) = root {
            let right_subtree = root.right.take();
            if let Some(right_subtree) = right_subtree {
                if self.get_height(&right_subtree.right) > self.get_height(&right_subtree.left) {
                    root.right = Some(right_subtree);                    
                } else { 
                    root.right = self.right_rotation(Some(right_subtree)); 
                 }
                self.left_rotation(Some(root))
            } else { None }
        } else { None }
    }
    
    /* Left Rotation: right subtree of right tree is imbalanced
       1                 2
        \               / \ 
         2      ->     1   3
          \
           3
    */
    fn left_rotation(&mut self, root: Nodeptr<T>) -> Nodeptr<T> {
        println!("Left Rotating....");
        match root {
            Some(mut root) => {
                let right_subtree = root.right.take();
                match right_subtree {
                    Some(mut right_subtree) => {
                        root.right = right_subtree.left.take();
                        self.update_height(&mut root);
                        right_subtree.left = Some(root);
                        self.update_height(&mut right_subtree);
                        Some(right_subtree)
                    }
                    None => None
                }                
            }
            None => None
        }
    }

    /* Right Rotation: left subtree of left tree is imbalanced
           3             2
          /             / \ 
         2      ->     1   3
        /
       1  
    */
    fn right_rotation(&mut self, root: Nodeptr<T>) -> Nodeptr<T> {
        println!("Right Rotating....");
        match root {
            Some(mut root) => {
                let left_subtree = root.left.take();
                match left_subtree {
                    Some(mut left_subtree) => {
                        root.left = left_subtree.right.take();
                        self.update_height(&mut root);
                        left_subtree.right = Some(root);
                        self.update_height(&mut left_subtree);
                        Some(left_subtree)
                    }
                    None => None
                }                
            }
            None => None
        }
    }

    pub fn delete(&mut self, value: T) {
        match self.root.take() {
            Some(root) => self.root = self.delete_helper_fuction(Some(root), value),
            None => {
                println!("No node to Delete");
            }
        }
    }

    fn delete_helper_fuction(&mut self, root: Nodeptr<T>, value: T) -> Nodeptr<T> {
        match root {
            Some(mut root) => {
                match root.value.cmp(&value) {            
                    Ordering::Greater => {
                        root.left = self.delete_helper_fuction(root.left, value);
                        self.update_height(&mut root);
                        self.rebalance(Some(root))
                    },
                    Ordering::Less => {
                        root.right = self.delete_helper_fuction(root.right, value);
                        self.update_height(&mut root);
                        self.rebalance(Some(root))
                    },
                    Ordering::Equal => self.delete_node(Some(root)),
                }
            },
            None => None
        }
    }

    fn delete_node(&self, root: Nodeptr<T>) -> Nodeptr<T> {
        match root {
            Some(root) => {
                match  (root.left, root.right) {
                    (Some(left_subtree), Some(right_subtree)) => {
                        todo!()
                    }
                    (Some(left_subtree), None) => Some(left_subtree),
                    (None, Some(right_subtree)) => Some(right_subtree),
                    _ => None
                }
            }
            None => None
        }
    }

    pub fn leaves(&mut self) -> usize { // return the number of leaves in tree
        match self.root.take() {
            Some(root) => self.leaves_helper_fuction(Some(root), 0),
            None => 0
        }
    }
    fn leaves_helper_fuction(&self, root: Nodeptr<T>, mut count: usize) -> usize {
        match root {
            Some(root) => {                
                if root.left == None && root.right == None {
                    count = count + 1;
                }
                let z = self.leaves_helper_fuction(root.left, count) + self.leaves_helper_fuction(root.right, count);
                count + z

            }
            None => 0
        }
    }
    pub fn inorder_traversal(&mut self) { // print in-order traversal of tree, implement iterator??
        match self.root.clone() {
            Some(root) => self.inorder_traversal_helper_fuction(Some(root)),
            None => ()
        }
    }
    fn inorder_traversal_helper_fuction(&self, root: Nodeptr<T>) {
        match root {
            Some(root) => {
                self.inorder_traversal_helper_fuction(root.left);
                //println!("{:?}", root.value);
                self.inorder_traversal_helper_fuction(root.right);
            },
            None => return
        }
    }

    fn get_height(&self, root: &Nodeptr<T>) -> usize { // Get size from nodeptr
        match root {
            Some(root) => root.height,
            None => 0
        }
    }
    fn balance_factor(&self, root: &Nodeptr<T>) -> isize { // Balance Factor = Height(Left_Tree) - Height(Right_Tree)
        match root {
            Some(root) => self.get_height(&root.left) as isize - self.get_height(&root.right) as isize,
            None => { println!{"BF panic!"}; panic!() },
        }
    }
    fn update_height(&self, root: &mut Node<T>) {  // update heights of nodes after rebalancing
        root.height = match (&root.left, &root.right) {
            (Some(l), Some(r)) => { max(l.height, r.height) + 1 }
            (Some(l), None) => { l.height + 1 }
            (None, Some(r)) => { r.height + 1 }    
            (None, None) => { 1 }
        }
    }
    pub fn tree_height(&self) -> usize { // return the height of the tree
        match self.root {
            Some(ref root) => root.height,
            None => 0
        }
    }
    pub fn empty(&self) -> bool { // check if tree is empty
        match self.root {
            Some(_) => false,
            None => true
        }
    }
    fn new_node(&self, value: T) -> Nodeptr<T> { // create a Node wrapped in Option and Box respectively
        Some(Box::new(Node::new(value)))
    }
}

// Tests //
#[test]
fn test_new_tree() {
    let tree: AVLTree<u32> = AVLTree::new();
    assert_eq!(tree.root, None);
}
#[test]
fn test_tree_height() {
    let tree: AVLTree<u32> = AVLTree::new();
    assert_eq!(tree.tree_height(), 0);
}
#[test]
fn test_empty_tree() {
    let tree: AVLTree<u32> = AVLTree::new();
    assert!(tree.empty());
}
#[test]
fn test_root_insert() {
    let mut tree: AVLTree<u32> = AVLTree::new();
    tree.insert(1);
    assert_eq!(tree, AVLTree { root: Some(Box::new(Node{ value: 1, left: None, right: None, height: 1 }))});
}
#[test]
fn test_get_height() {
    let mut tree: AVLTree<u32> = AVLTree::new();
    assert_eq!(tree.get_height(&tree.root), 0);
    tree.insert(2);
    assert_eq!(tree.get_height(&tree.root), 1);
    tree.insert(1);
    tree.insert(3);
    assert_eq!(tree.get_height(&tree.root), 2);
    tree.insert(4);
    assert_eq!(tree.get_height(&tree.root), 3);
}
#[test]
#[should_panic]
fn test_balance_factor() {
    let mut tree: AVLTree<u32> = AVLTree::new();
    assert_eq!(tree.balance_factor(&tree.root), 999); // Will Panic
    tree.insert(2);
    tree.insert(1);
    tree.insert(3);
    tree.insert(4);
    assert_eq!(tree.balance_factor(&tree.root), -1);
}
#[test]
fn test_test() { // Test of anything
    let mut tree: AVLTree<u32> = AVLTree::new();
    tree.insert(3);
    tree.insert(4);
    tree.insert(1);
    tree.insert(2);
}

