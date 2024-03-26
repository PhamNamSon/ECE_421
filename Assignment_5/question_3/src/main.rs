use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }

    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs
                .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }

    fn length(&self) -> usize {
        fn count_nodes(node: &TrieNode) -> usize {
            let mut count = 0;
            if node.value.is_some() {
                count += 1;
            }
            for child in node.chs.values() {
                count += count_nodes(child);
            }
            count
        }

        count_nodes(&self.root)
    }

    fn iter(&self) -> Vec<(String, Option<i32>)> {
        fn collect_nodes(prefix: String, node: &TrieNode, result: &mut Vec<(String, Option<i32>)>) {
            if let Some(value) = node.value {
                result.push((prefix.clone(), Some(value)));
            }
            for (&c, child) in &node.chs {
                let new_prefix = format!("{}{}", prefix, c);
                collect_nodes(new_prefix, child, result);
            }
        }

        let mut result = Vec::new();
        collect_nodes(String::new(), &self.root, &mut result);
        result
    }

    fn find(&self, key: &String) -> Option<&TrieNode> {
        let mut current_node = &self.root;
        for c in key.chars() {
            match current_node.chs.get(&c) {
                Some(node) => current_node = node,
                None => return None,
            }
        }
        Some(current_node)
    }

    fn delete(&mut self, key: &String) -> Option<i32> {
        let mut current_node = &mut self.root;
        let mut nodes_stack: Vec<(*mut TrieNode, char)> = Vec::new();
    
        for c in key.chars() {
            if let Some(node) = current_node.chs.get_mut(&c) {
                nodes_stack.push((node as *mut TrieNode, c));
                current_node = node;
            } else {
                return None;
            }
        }
    
        let value = current_node.value.take();
        if current_node.chs.is_empty() {
            while let Some((node_ptr, c)) = nodes_stack.pop() {
                let node = unsafe { &mut *node_ptr };
                if node.value.is_none() && node.chs.len() == 1 {
                    node.chs.remove(&c);
                } else {
                    break;
                }
            }
        }
    
        value
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Bar".to_string(), 2);

    println!("{:#?}", trie);
    println!("\n-----------------------------------------------------------------------\n");

    println!("Length: {}", trie.length());
    println!("\n-----------------------------------------------------------------------\n");

    println!("{:#?}", trie.iter());
    println!("\n-----------------------------------------------------------------------\n");

    println!("{:#?}", trie.find(&"Bar".to_string()));
    println!("\n-----------------------------------------------------------------------\n");

    println!("Before delete: {:#?}", trie);
    let value = trie.delete(&"Bar".to_string());
    println!("Deleted value: {:?}", value);
    println!("After delete: {:#?}", trie);

}