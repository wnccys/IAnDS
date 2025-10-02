const ORDER: usize = 4;
const MAX_LEAF_CAP: usize = ORDER - 1;

trait Tree {
    fn insert(id: usize, name: String) -> ();
}

struct Root([Option<Box<dyn Node>>; ORDER]);

struct Internal<'a> {
    left: &'a dyn Node,
    right: &'a dyn Node,
}

struct Leaf {
    key: usize,
    value: String,
}

trait Node {}

fn main() {
    println!("Hello, world!");
}
