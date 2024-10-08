use std::collections::HashMap;

use crate::enumerate_duplicates;

use super::huff::*;

/// ENCODING HELPER FUNCTIONS

// A node in the Huffman Tree
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EncodeNode {
    pub frequency: usize,
    pub byte: Option<u8>,
    pub left: Option<Box<EncodeNode>>,
    pub right: Option<Box<EncodeNode>>,
}

/// Implementing PartialOrd manually is required since
/// the derive macro lexicographically orders the fields of the struct,
/// which is NOT what we want
impl PartialOrd for EncodeNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Instead, we define our own Ordering for a HuffNode.
/// 2 Nodes are compared in the following order of precedence
///
/// 1. Their frequencies are compared
/// 2. Internal/Leaf status. Internal < Leaf
/// 3. Byte value associated with the Leaf (in the case of Leaf vs Leaf)
impl Ord for EncodeNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.frequency != other.frequency {
            self.frequency.cmp(&other.frequency)
        } else {
            use std::cmp::Ordering;
            match (self.byte, other.byte) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(b1), Some(b2)) => b1.cmp(&b2),
            }
        }
    }
}

/// Recursive backtracking function to trace and return the path to a given byte
/// in the Huffman tree
pub fn encode_byte(node: &Option<Box<EncodeNode>>, byte: u8, path: &mut Vec<u8>) -> bool {
    if let Some(ref n) = node {
        match n.byte {
            Some(b) => b == byte,
            None => {
                let left = encode_byte(&n.left, byte, path);
                let right = encode_byte(&n.right, byte, path);
                if left {
                    path.push(0);
                } else if right {
                    path.push(1);
                }
                return left || right;
            }
        }
    } else {
        false
    }
}

/// DECODING HELPER FUNCTIONS

/// Constructs a Preorder array of nodes, where `(Some(u8), None) = (Leaf(u8), Internal)`
pub fn pre_order(node: &Option<Box<EncodeNode>>, arr: &mut Vec<Option<u8>>) {
    if let Some(ref n) = node {
        arr.push(n.byte);
        pre_order(&node.as_ref().unwrap().left, arr);
        pre_order(&node.as_ref().unwrap().right, arr);
    }
}

/// Constructs an Inorder array of nodes, where `(Some(u8), None) = (Leaf(u8), Internal)`
pub fn in_order(node: &Option<Box<EncodeNode>>, arr: &mut Vec<Option<u8>>) {
    if let Some(ref n) = node {
        in_order(&node.as_ref().unwrap().left, arr);
        arr.push(n.byte);
        in_order(&node.as_ref().unwrap().right, arr);
    }
}

#[derive(Debug)]
pub struct ReconstructNode {
    pub val: usize,
    pub left: Option<Box<ReconstructNode>>,
    pub right: Option<Box<ReconstructNode>>,
}

pub fn build_tree(preorder: Vec<usize>, inorder: Vec<usize>) -> Option<Box<ReconstructNode>> {
    let mut root = ReconstructNode {
        val: preorder[0],
        left: None,
        right: None,
    };
    // Split the inorder vec
    println!("Looking for {}", preorder[0]);
    let i = inorder.iter().position(|&x| x == preorder[0])?;
    println!("Found i = {i}");
    let inorder_left = inorder[0..i].to_vec();
    let inorder_right = inorder[i + 1..].to_vec();

    println!("Left, Right = {:?}, {:?}", inorder_left, inorder_right);
    // Split the preorder vec
    let preorder_left = preorder[1..1 + inorder_left.len()].to_vec();
    let preorder_right = preorder[1 + inorder_left.len()..].to_vec();

    if inorder_left.len() > 0 {
        root.left = build_tree(preorder_left, inorder_left);
    }
    if inorder_right.len() > 0 {
        root.right = build_tree(preorder_right, inorder_right);
    }

    Some(Box::new(root))
}

pub fn map_to_reconstruct(
    preorder: Vec<Option<u8>>,
    inorder: Vec<Option<u8>>,
) -> (Vec<usize>, Vec<usize>) {
    let preorder = enumerate_duplicates(preorder);
    let inorder = enumerate_duplicates(inorder);

    let mut map: HashMap<(Option<u8>, usize), usize> = HashMap::new();

    for (index, &elem) in inorder.iter().enumerate() {
        map.insert(elem, index);
    }

    (
        preorder
            .iter()
            .map(|elem| *map.get(elem).unwrap())
            .collect(),
        (0..inorder.len()).collect(),
    )
}
