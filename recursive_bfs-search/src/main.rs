use std::collections::HashMap;

/// Breadth First Search implementation
/// 
pub fn bfs_search<'a>(
    graph: &'a HashMap<&str, Vec<&str>>,
    target: &str,
    queue: &mut Vec<&'a str>,
    processed: &mut Vec<&'a str>
) -> Option<&'a str> {
    // Base case
    if queue.len() == 0 { return None };

    let person = queue.pop().unwrap();
    processed.push(person);

    let friends = match graph.get(person) {
        Some(vec) => vec,
        _ => &Vec::new(),
    };

    // Check if friends are present and are no in processed
    if friends.contains(&target) {
        return Some(person);
    }

    // Add new friends to queue
    for &person in friends {
        if !processed.contains(&person) { queue.push(person) }
    }

    bfs_search(graph, target, queue, processed)
}