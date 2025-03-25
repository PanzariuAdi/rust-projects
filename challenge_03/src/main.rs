use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Node {
    pub letter: Option<char>,
    pub occurences: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(letter: char, occurences: usize) -> Self {
        Node {
            letter: Some(letter),
            occurences,
            left: None,
            right: None,
        }
    }

    fn new_node(occurences: usize, left_node: Node, right_node: Node) -> Self {
        Node {
            letter: None,
            occurences,
            left: Some(Box::new(left_node)),
            right: Some(Box::new(right_node)),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.occurences == other.occurences
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.occurences.cmp(&other.occurences)
    }
}

fn open_file(path: &str) -> Result<BufReader<File>, String> {
    let file = File::open(path).map_err(|_| "Error at opening the file".to_string())?;

    Ok(BufReader::new(file))
}

fn build_occurence_map(reader: Result<BufReader<File>, String>) -> HashMap<char, usize> {
    let mut occurency_map: HashMap<char, usize> = HashMap::new();

    reader.unwrap().lines().for_each(|line| match line {
        Ok(line_str) => {
            for ch in line_str.chars() {
                occurency_map.insert(ch, occurency_map.get(&ch).unwrap_or(&0) + 1);
            }
        }
        Err(_) => {
            println!("Error at reading the line");
        }
    });

    occurency_map
}

fn build_binary_heap(occurence_map: HashMap<char, usize>) -> BinaryHeap<Node> {
    let mut pq = BinaryHeap::new();

    for (letter, occurences) in occurence_map.iter() {
        let _ = pq.push(Node::new(*letter, *occurences));
    }

    pq
}

fn build_huffman_tree(binary_heap: &mut BinaryHeap<Node>) -> Node {
    while binary_heap.len() > 1 {
        let tmp1 = binary_heap.pop().unwrap();
        let tmp2 = binary_heap.pop().unwrap();

        let new_node = Node::new_node(tmp1.occurences + tmp2.occurences, tmp1, tmp2);
        binary_heap.push(new_node);
    }

    binary_heap.pop().unwrap()
}

fn get_encoding_map(node: &Node, current_encoding: String, chars: &mut HashMap<char, String>) {
    if let Some(ch) = node.letter {
        chars.insert(ch, current_encoding);
    } else {
        if let Some(ref left) = node.left {
            get_encoding_map(left, format!("{}0", current_encoding), chars);
        }
        if let Some(ref right) = node.right {
            get_encoding_map(right, format!("{}1", current_encoding), chars);
        }
    }
}

fn encode_message(message: String, chars: &mut HashMap<char, String>) -> String {
    let mut encoded_message = String::from("");

    for ch in message.chars() {
        encoded_message.push_str(chars.get(&ch).unwrap());
    }

    encoded_message.to_string()
}

fn main() {
    let reader = open_file("test.txt");
    let occurency_map = build_occurence_map(reader);
    let mut binary_heap = build_binary_heap(occurency_map);
    let huffman_tree = build_huffman_tree(&mut binary_heap);

    let mut encoding_chars = HashMap::new();
    get_encoding_map(&huffman_tree, "".to_string(), &mut encoding_chars);

    for (key, value) in encoding_chars {
        println!("{}: {}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_occurence_map() {
        let reader = open_file("test.txt");
        let occurency_map = build_occurence_map(reader);

        assert_eq!(occurency_map.get(&'X'), Some(333).as_ref());
        assert_eq!(occurency_map.get(&'t'), Some(223000).as_ref());
    }

    #[test]
    fn test_build_huffman_tree() {
        let mut binary_heap = build_binary_heap_sample();
        let root = build_huffman_tree_sample();
        assert_eq!(root, build_huffman_tree(&mut binary_heap));
    }

    #[test]
    fn test_build_encoding_map() {
        let root = build_huffman_tree_sample();

        let mut chars = HashMap::new();
        get_encoding_map(&root, "".to_string(), &mut chars);

        assert_eq!(chars.get(&'C'), Some(&"1110".to_string()));
        assert_eq!(chars.get(&'D'), Some(&"101".to_string()));
        assert_eq!(chars.get(&'E'), Some(&"0".to_string()));
    }

    #[test]
    fn test_encode_message() {
        let root = build_huffman_tree_sample();
        let mut chars = HashMap::new();
        get_encoding_map(&root, "".to_string(), &mut chars);

        let actual_encoded = encode_message("MELCDULKUMZ".to_string(), &mut chars);
        let expected_encoded = "111110110111010110011011110110011111111100".to_string();

        assert_eq!(expected_encoded, actual_encoded);
    }

    fn build_binary_heap_sample() -> BinaryHeap<Node> {
        let mut binary_heap = BinaryHeap::new();

        binary_heap.push(Node::new('C', 32));
        binary_heap.push(Node::new('D', 42));
        binary_heap.push(Node::new('E', 120));
        binary_heap.push(Node::new('K', 7));
        binary_heap.push(Node::new('L', 42));
        binary_heap.push(Node::new('M', 24));
        binary_heap.push(Node::new('U', 37));
        binary_heap.push(Node::new('Z', 2));

        binary_heap
    }

    fn build_huffman_tree_sample() -> Node {
        let z_leaf = Node::new('Z', 2);
        let u_leaf = Node::new('U', 37);
        let m_leaf = Node::new('M', 24);
        let l_leaf = Node::new('L', 42);
        let k_leaf = Node::new('K', 7);
        let e_leaf = Node::new('E', 120);
        let d_leaf = Node::new('D', 42);
        let c_leaf = Node::new('C', 32);

        let node_9 = Node::new_node(9, z_leaf, k_leaf);
        let node_33 = Node::new_node(33, node_9, m_leaf);
        let node_65 = Node::new_node(65, c_leaf, node_33);
        let node_107 = Node::new_node(107, l_leaf, node_65);
        let node_79 = Node::new_node(79, u_leaf, d_leaf);
        let node_186 = Node::new_node(186, node_79, node_107);
        let node_306 = Node::new_node(306, e_leaf, node_186);

        node_306
    }
}
