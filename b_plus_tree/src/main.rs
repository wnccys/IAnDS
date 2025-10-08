const ORDER: usize = 4;
const MAX_LEAF_CAP: usize = ORDER - 1;
const MAX_INTERNAL_CAP: usize = ORDER;

type Key = usize;
type Data = (usize, String);

/// Root is the main entry point of the graph, it can be either a internal of a leaf.
/// It has a collection of Nodes (max. ORDER - 1) which are elevated into a new root when the limit is reached.
/// 
struct Root<'a>(Option<Node<'a>>);

impl<'a> Root<'a> {
    fn new(data: (usize, String)) -> Self {
        Root {
            0: Some(Node::Leaf(Leaf::new(data))),
        }
    }
}

impl<'a> Root<'a> {
    fn insert(&mut self, data: Data) {
        match self.0.take().unwrap() {
            Node::Internal(mut intr) => intr.insert(data),
            // Means self has only a leaf as value
            Node::Leaf(mut leaf) => {
                leaf.insert(data);

                if leaf.is_full() { self.split_leaf(leaf); }

                true
            },
        };
    }

    fn split_leaf(&mut self, leaf: Leaf<'a>) {
        // Order tends (MUST AT LEAST) always to be a pair,
        // and the first item on the right side of overflow becomes root;
        let middle_idx = (leaf.0.len() - 1) / 2;
        let new_root_id = leaf.0[middle_idx + 1]
            .as_ref()
            .unwrap()
            .0
            .clone();

        let mut l2 = Leaf { 0: [const { None }; MAX_LEAF_CAP], 1: None };
        l2.0.clone_from_slice(&leaf.0[middle_idx + 1 ..]);

        let mut l1 = Leaf { 0: [const { None }; MAX_LEAF_CAP], 1: None };
        l1.0.clone_from_slice(&leaf.0[0..=middle_idx]);

        // Root is now internal!
        let new_root = Internal::new(
            new_root_id, 
            l1,
            l2
        );

        self.0 = Some(Node::Internal(new_root));
    }

    fn split_internal() {}

    fn inspect(&self) {}
}

enum Node<'a> {
    Internal(Internal<'a>),
    Leaf(Leaf<'a>),
}

impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Internal has a collection of key-pointer pairs to it's children;
/// 
struct Internal<'a> {
    // ORDER + 1 because of the last pointer to the last section of childs
    refs: [Option<(Key, Box<Node<'a>>)>; MAX_INTERNAL_CAP + 1]
}

// REVIEW new values ptrs
impl<'a> Internal<'a> {
    fn new(id: usize, l1: Leaf<'a>, l2: Leaf<'a>) -> Self {
        let mut refs: _ = [ const { None } ; _];

        // REVIEW Uses first item Id as the Id of value slice
        refs[0] = Some((l1.0[0].as_ref().unwrap().0, Box::new(l1.into())));
        // Id is assigned by the first right slice Id, the new root
        refs[1] = Some((id, Box::new(l2.into())));

        Internal { refs }
    }

    fn insert(&mut self, data: Data) -> bool {
        todo!()
    }

    fn is_full(&self) -> bool {
        // O(n) >:^(|)
        self
        .refs
        .iter()
        .all(|node| node.is_some())
    }
}

/// Leaf is a vectored linked key-value pair on the sharp of graph with a pointer
/// to the next first element on (that's why linked); 
/// 
struct Leaf<'a>([Option<Data>; MAX_LEAF_CAP], Option<&'a [Option<Data>]>);
impl Leaf<'_> {
    fn new(data: Data) -> Self {
        let mut items: [Option<Data>; MAX_LEAF_CAP] = [const { None }; MAX_LEAF_CAP];
        items[0] = Some(data);
        
        Leaf {
            0: items,
            1: None
        }
    }

    fn insert(&mut self, data: Data) {
        let idx = self.0
        .iter()
        .position(|node| node.is_none())
        .expect("Could not find valid position for Leaf insert.");

        self.0[idx] = Some(data);
    }

    fn is_full(&self) -> bool {
        self.0
        .iter()
        .all(|node| node.is_some())
    }
}

impl<'a> Into<Node<'a>> for Leaf<'a> {
    fn into(self) -> Node<'a> {
        Node::Leaf(self)
    }
}

fn main() {
    let data: [(usize, String); _] = [
        (1, "Alice".to_string()),
        (2, "Bob".to_string()),
        (3, "Charlie".to_string()),
        (4, "David".to_string()),
        (5, "Eve".to_string()),
        (6, "Frank".to_string()),
        (7, "Grace".to_string()),
        (8, "Heidi".to_string()),
        (9, "Ivan".to_string()),
        (10, "Judy".to_string()),
    ];

    let mut root = Root::new(data[0].clone());

    root.insert(data[1].clone());
    root.insert(data[2].clone());
    root.insert(data[3].clone());
    root.inspect();
}
