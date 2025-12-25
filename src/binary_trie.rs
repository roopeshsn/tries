struct BinaryTrieNode {
    is_zero: Option<Box<BinaryTrieNode>>,
    is_one: Option<Box<BinaryTrieNode>>,
    is_end: bool,
}

impl BinaryTrieNode {
    fn new() -> Self {
        Self{
            is_zero: None,
            is_one: None,
            is_end: false,
        }
    } 
}

struct BinaryTrie {
    root: Box<BinaryTrieNode>,
}

impl BinaryTrie {
    fn new() -> Self {
        Self{
            root: Box::new(BinaryTrieNode::new()),
        }
    } 

    fn insert(&mut self, binary_value: &str) {
        let mut current_node = &mut self.root;
        for bit in binary_value.as_bytes() {
            if *bit == b'0' {
                if current_node.is_zero.is_none() {
                    current_node.is_zero = Some(Box::new(BinaryTrieNode::new()));
                }
                current_node = current_node.is_zero.as_mut().unwrap();
            } else if *bit == b'1' {
                if current_node.is_one.is_none() {
                    current_node.is_one = Some(Box::new(BinaryTrieNode::new()));
                }
                current_node = current_node.is_one.as_mut().unwrap();
            } else {
                panic!("Invalid bit in input: must be '0' or '1'");
            }
        }
        current_node.is_end = true;
    }

    fn search(&self, binary_value: &str) -> bool {
        let mut current_node = &self.root;
        for bit in binary_value.as_bytes() {
            if *bit == b'0' {
                if let Some(ref zero_node) = current_node.is_zero {
                    current_node = zero_node;
                } else {
                    return false;
                }
            } else if *bit == b'1' {
                if let Some(ref one_node) = current_node.is_one {
                    current_node = one_node;
                } else {
                    return false;
                }
            } else {
                panic!("Invalid bit in input: must be '0' or '1'");
            }
        }
        current_node.is_end
    }
}

// input: "00001010" or 10 in decimal

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_search_binary() {
        let mut trie = BinaryTrie::new();

        trie.insert("00001010"); // 10 in decimal
        assert!(trie.search("00001010"));
    }
}
