use std::io;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug)]
enum Node {
    Leaf(u8),
    Internal(Box<Node>, Box<Node>),
}

#[derive(Debug)]
struct HeapNode {
    freq: u32,
    node: Node,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for HeapNode {}

fn build_huffman_tree(freqs: &[u32; 256]) -> Option<Node> {
    let mut heap = BinaryHeap::new();

    for (i, &freq) in freqs.iter().enumerate() {
        if freq > 0 {
            heap.push(HeapNode { freq, node: Node::Leaf(i as u8) });
        }
    }

    if heap.is_empty() {
        return None;
    }

    if heap.len() == 1 {
        let node = heap.pop().unwrap();
        return Some(node.node);
    }
    
    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();

        let merged = HeapNode { freq: n1.freq + n2.freq, node: Node::Internal(Box::new(n1.node), Box::new(n2.node)) };
        heap.push(merged);
    }
    
    Some(heap.pop().unwrap().node)
}

fn build_codes(node: &Node, prefix: Vec<bool>, codes: &mut HashMap<u8, Vec<bool>>) {
    match node {
        Node::Leaf(byte) => {
            codes.insert(*byte, prefix);
        }
        Node::Internal(left, right) => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            build_codes(left, left_prefix, codes);
            let mut right_prefix = prefix.clone();
            right_prefix.push(true);
            build_codes(right, right_prefix, codes);
        }
    }
}

fn main() -> io::Result<()> {
    let path = "images.bmp";
    let data = fs::read(path)?;

    let mut freqs = [0u32; 256];
    for &byte in &data {
        freqs[byte as usize] += 1;
    }

    let root = match build_huffman_tree(&freqs) {
        Some(tree) => tree,
        None => {
            println!("Failed to build Huffman tree");
            return Ok(());
        },
    };

    let mut codes: HashMap<u8, Vec<bool>> = HashMap::new();
    build_codes(&root, Vec::new(), &mut codes);

    for (byte, code) in codes.iter() {
        println!("{:3} (0x{:02X}):", byte, byte);
        for bit in code {
            print!("{}", if *bit { 1 } else { 0 });
        }
        println!();
    }

    Ok(())
}
