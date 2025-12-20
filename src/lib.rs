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

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.children.contains_key(&ch) {
                let new_node = TrieNode::new();
                node.children.insert(ch, new_node);
            }

            node = node.children.get_mut(&ch).unwrap();
        }

        node.end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;

        for ch in word.chars() {
            if let Some(children) = node.children.get(&ch) {
                node = children;
            } else {
                return false;
            }
        }

        node.end_of_word
    }

    fn find_all_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        let mut node = &self.root;
        for ch in prefix.chars() {
            if let Some(children) = node.children.get(&ch) {
                node = children;
            } else {
                return result;
            }
        }

        
        let mut current = prefix.to_string();
        Self::collect_words(node, &mut current, &mut result);

        println!("{:?}", result);

        return result;
    }

    fn collect_words(node: &TrieNode, current: &mut String, result: &mut Vec<String>) {
        if node.end_of_word {
            result.push(current.clone());
        }

        for (ch, child) in &node.children {
            current.push(*ch);
            Self::collect_words(child, current, result);
            current.pop();
        }
}

}

// TODO:
// 1. Pretty print
// 2. Traversal method to print every word in the trie.
// 3. Method to print words that starts with a prefix.
// 4. Delete method
// 5. Method to print the memory usage of a trie

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_search_word() {
        let mut trie = Trie::new();

        trie.insert("cat");
        trie.insert("dog");

        assert!(trie.search("cat"));
        assert!(trie.search("dog"));
        assert!(!trie.search("cow"));
    }

    #[test]
    fn prefix_is_not_word() {
        let mut trie = Trie::new();

        trie.insert("car");

        assert!(!trie.search("ca"));
        assert!(trie.search("car"));
    }

    #[test]
    fn multiple_words_shared_prefix() {
        let mut trie = Trie::new();

        trie.insert("car");
        trie.insert("cart");
        trie.insert("carbon");

        assert!(trie.search("car"));
        assert!(trie.search("cart"));
        assert!(trie.search("carbon"));
        assert!(!trie.search("cars"));
    }

    #[test]
    fn empty_trie_search() {
        let trie = Trie::new();

        assert!(!trie.search("anything"));
    }

    #[test]
    fn words_with_prefix_ap() {
        let mut trie = Trie::new();

        trie.insert("app");
        trie.insert("apple");
        trie.insert("apply");
        trie.insert("apt");
        trie.insert("bat");

        let mut words = trie.find_all_with_prefix("ap");

        // Order is not guaranteed because HashMap is unordered
        words.sort();

        assert_eq!(
            words,
            vec!["app", "apple", "apply", "apt"]
        );
    }
}
