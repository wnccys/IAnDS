use std::collections::VecDeque;

/// Minimal BFS graph algorithm implementation
/// 
/// This searches an certain name and uses an Id in order to identify each Node (only miscellaneouslly);
/// 
/// The mocks below

#[derive(Debug)]
pub struct Node<'a> {
    pub id: usize,
    pub name: String,
    pub neighbors: Option<Vec<&'a Node<'a>>>
}

impl<'a> Node<'a> {
    fn new(id: usize, name: String, neighbors: Option<Vec<&'a Node>>) -> Self {
        Node {
            id,
            name,
            neighbors
        }
    }
}

// test for n4, the only pointing to n9
// test for n8 search, it is the only order 4 search (need layer 3 to be found);

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: [PROGRAM] -- [Search Name]");
    }
    
    // l3 relations
    let n9 = Node::new(9, "Poseidon".to_string(), None);
    let n8 = Node::new(8, "Gaea".to_string(), None);
    let n7 = Node::new(7, "Chaos".to_string(), Some(vec![&n8]));
    let n6 = Node::new(6, "Uranus".to_string(), None);

    // l2 relations
    let n5 = Node::new(5, "Ares".to_string(), Some(vec![&n7, &n6]));
    let n4 = Node::new(4, "Atenas".to_string(), Some(vec![&n9]));
    let n3 = Node::new(3, "Apollo".to_string(), Some(vec![&n6]));

    // l1 relations
    let n2 = Node::new(2, "Hera".to_string(), Some(vec![&n5]));
    let n1 = Node::new(1, "Zeus".to_string(), Some(vec![&n3, &n4]));

    let root = Node::new(0, "Fate".to_string(), Some(vec![&n1, &n2]));
    
    let mut queue = VecDeque::new();
    queue.push_front(&root);

    let mut processed = vec![];

    let result = bfs_search(&args[1][..], &mut queue, &mut processed);

    match result {
        Some(target) => {
            println!("Found: {:?}", target);
        }
        None => println!("Could not find any item named {}!", &args[1][..])
    }
}

/// Breadth-First Search impl
/// 
pub fn bfs_search<'a>(
    target: &'_ str,
    queue: &'a mut VecDeque<&'_ Node<'_>>,
    processed: &'_ mut Vec<usize>
) -> Option<&'a Node<'a>> {
    // Base case
    if queue.is_empty() { return None };

    // Remove [Self] Node from start of processing queue HERE -> [..]
    let node = queue.pop_front().unwrap();

    // Names comparison are normalized, so poseidon matches with Poseidon
    if node.name.to_lowercase() == target.to_lowercase() { return Some(node); }

    // Set neighbors to end of queue
    if node.neighbors.is_some() {
        // Filters processed ones
        for neighbor in node.neighbors.as_ref().unwrap()
            .iter()
            .filter(|node| !processed.contains(&node.id)) 
        {
            // Add neighbor to end of queue HERE [..] <-
            queue.push_back(neighbor);
        }

        // Set current node as processed
        processed.push(node.id);
    }

    // Re-init search, until find a name or falling back to None, when no entry was found
    return bfs_search(target, queue, processed);
}