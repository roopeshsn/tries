use std::collections::HashMap;

// A Trie can be intialized like below:
// main() {
//   trie = Trie()
//   trie.insert()
//   trie.search()
// }

struct TrieNode {
    children: HashMap<char, TrieNode>, // TriedNode should be wrapped with Box smart pointer
    end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self{
            children: HashMap::new(),
            end_of_word: false,
        }
    } 
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self{
            root: TrieNode::new(),
        }
    } 

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.children.contains_key(&ch) {
                let new_node = TrieNode::new();
                node.children.insert(ch, new_node);
            }

            node = node.children.get_mut(&ch).unwrap();
        }
    }
}