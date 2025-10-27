/// Djikstra graph transverse algorithm impl
/// 
/// https://www.youtube.com/shorts/0O2oVlTmnqY

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node<'a> {
    pub name: String,
    // (childs, cost)
    pub childs: Vec<(&'a Node<'a>, usize)>
}

impl<'a> Node<'a> {
    fn new(name: String, childs: Vec<(&'a Node, usize)>) -> Self {
        Node {
            name,
            childs
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let target = &args[1];

    if args.len() < 2 {
        eprintln!("Usage: [PROGRAM] -- [Search Term]");
    }
    
    // l3 relations
    let n5 = Node::new("The Fates".to_string(), vec![]);

    // l2 relations
    let n3 = Node::new("Atenas".to_string(), vec![(&n5, 20)]);
    let n4 = Node::new("Cronos".to_string(), vec![(&n5, 10)]);

    // l1 relations
    let n1 = Node::new("Hera".to_string(), vec![(&n3, 15), (&n4, 20)]);
    let n2 = Node::new("Zeus".to_string(), vec![(&n3, 30), (&n4, 35)]);

    let root = Node::new("Root".to_string(), vec![(&n1, 5), (&n2, 0)]);

    let mut costs = HashMap::<String, usize>::default();

    // (Node name, ?Link to parent)
    // Relation stores the current most cheap parent relation of a child
    let mut relations = HashMap::<String, Option<&'_ Node>>::new();

    djkt_search(&root, &mut costs, &mut relations);

    if costs.get(target).is_none() { panic!("Term not found!"); };

    println!("Resolving {}...", target);
    resolve_term(target, &costs, &relations);
    println!("Total Cost: {}", costs.get(target).unwrap());
}

/// Trace target cost and parents
/// 
fn resolve_term<'a>(
    target: &str,
    costs: &'_ HashMap::<String, usize>,
    relations: &'_ HashMap::<String, Option<&'a Node<'a>>>,
) {
    match costs.get(target) {
        Some(cost) => {
            print!("{} [{}] -> ", target, cost);
        }
        None => {
            println!("Root [X]");
            // base case;
            return;
        }
    }

    resolve_term(
        &relations.get(target).expect("Invalid node Parent state").unwrap().name,
        costs,
        relations
    );
}

fn djkt_search<'a>(
    node: &'a Node,
    costs: &'_ mut HashMap::<String, usize>,
    relations: &'_ mut HashMap::<String, Option<&'a Node<'a>>>,
) {
    // No base case is needed !!
    let node_cost = costs.get(&node.name).unwrap_or(&0).clone();

    for child in node.childs.iter() {
        // Check if is there a cheapest way to reach the child node by getting it on relations 
        // (ensuring no relation is best than this one), if it is the best one, update costs on relations,
        // so relations[child].parent :: == (child.0) returns the new parent (this let node)
        // if matches when child has a parent set already;
        // Anti Borrow-Checker Scope xD so Rust knows the struct is not mutably borrowed anymore;
        {
            if relations.get(&child.0.name).is_some() {
                // True if current child relation cost is better than old relation;
                // This means that tracing the cost of this child we're working with is a must, in order to validate this assumption;
                // What means we must trace each parent until root, to find this current cost;
                // Trace parent cost + possible cheapest child;
                if (node_cost + child.1) < *(costs.get(&child.0.name).unwrap_or(&0)) {
                    costs.insert(child.0.name.clone(), child.1 + node_cost);

                    relations.insert(child.0.name.clone(), Some(node));
                };
            // Child node has no relations yet
            } else {
                // Cost is updated
                costs.insert(child.0.name.clone(), node_cost + child.1);
                // Set current node as it's parent
                relations.insert(child.0.name.clone(), Some(node));
            };
        }

        djkt_search(child.0, costs, relations);
    }
}