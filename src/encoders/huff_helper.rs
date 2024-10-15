

/// ENCODING HELPER FUNCTIONS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node {
    Leaf(u8),
    Internal(u8),
}

// A node in the Huffman Tree
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HuffmanNode {
    pub frequency: usize,
    pub byte: Node,
    pub left: Option<Box<HuffmanNode>>,
    pub right: Option<Box<HuffmanNode>>,
}

/// Implementing PartialOrd manually is required since
/// the derive macro lexicographically orders the fields of the struct,
/// which is NOT what we want
impl PartialOrd for HuffmanNode {
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
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.frequency != other.frequency {
            self.frequency.cmp(&other.frequency)
        } else {
            use super::huff_helper::Node::*;
            use std::cmp::Ordering;
            match (self.byte, other.byte) {
                (Leaf(b1), Leaf(b2)) => b1.cmp(&b2),
                (Leaf(_), Internal(_)) => Ordering::Greater,
                (Internal(_), Leaf(_)) => Ordering::Less,
                (Internal(_), Internal(_)) => Ordering::Equal,
            }
        }
    }
}

/// Recursive backtracking function to trace and return the path to a given byte
/// in the Huffman tree
pub fn encode_byte(node: &Option<Box<HuffmanNode>>, byte: u8, path: &mut Vec<u8>) -> bool {
    if let Some(ref n) = node {
        match n.byte {
            Node::Leaf(b) => b == byte,
            Node::Internal(_) => {
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
pub fn pre_order(node: &Option<Box<HuffmanNode>>, arr: &mut Vec<Node>) {
    if let Some(ref n) = node {
        arr.push(n.byte);
        pre_order(&node.as_ref().unwrap().left, arr);
        pre_order(&node.as_ref().unwrap().right, arr);
    }
}

/// Constructs an Inorder array of nodes, where `(Some(u8), None) = (Leaf(u8), Internal)`
pub fn in_order(node: &Option<Box<HuffmanNode>>, arr: &mut Vec<Node>) {
    if let Some(ref n) = node {
        in_order(&node.as_ref().unwrap().left, arr);
        arr.push(n.byte);
        in_order(&node.as_ref().unwrap().right, arr);
    }
}


pub fn build_tree(preorder: &[Node], inorder: &[Node]) -> Option<Box<HuffmanNode>> {
    if preorder.len() == 0 || inorder.len() == 0 {
        return None;
    }
    let mut root = Some(Box::new(HuffmanNode {
        byte: preorder[0],
        frequency: 0, // Frequency is not used when decoding, so we set it to 0.
        left: None,
        right: None,
    }));

    let mid = inorder.iter().position(|r| *r == preorder[0]).unwrap();
    root.as_mut().unwrap().left = build_tree(&preorder[1..mid + 1], &inorder[0..mid]);
    root.as_mut().unwrap().right = build_tree(&preorder[mid + 1..], &inorder[mid + 1..]);

    return root;
}
