use std::collections::HashMap;

/// Djikstra Algorithm implementation
/// 
pub fn djkt_search(
    parents: &mut HashMap<&str, Vec<&str>>,
    costs: &mut HashMap<&str, u16>,
    to_be_processed: &mut Vec<String>
) {
    if to_be_processed.len() == 0 { return }

    let parent = &to_be_processed[0];

    let childs = 
        parents
        .get(parent.as_str())
        .expect(&format!("Error: Could not get parent: {}", parent));

    for &child in childs {
    }

    djkt_search(parents, costs, to_be_processed);
}

// mod cap7;
// use cap7::djkt_search;
// fn main() {
//     // This simulates an ideal scenario where the keys are present exactly as the code
//     let mut parents: HashMap<&str, Vec<&str>> = HashMap::new(); 
//     parents.insert("a", vec!["b", "c"]);
//     parents.insert("b", vec!["d"]);
//     parents.insert("c", vec!["b", "e"]);
//     parents.insert("d", vec!["e"]);

//     let mut costs: HashMap<&str, u16> = HashMap::new();
//     costs.insert("a", 0);

//     let mut to_be_processed: Vec<String> = vec![];
    
//     for &nodes in parents.keys()  {
//         to_be_processed.push(nodes.to_owned());
//     }

//     djkt_search(&mut parents, &mut costs, &mut to_be_processed);