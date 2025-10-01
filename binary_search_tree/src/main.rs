type Child<'a> = &'a Node<'a>;

#[derive(Debug)]
struct Node<'a> {
    value: i32,
    // (lower, higher)
    children: Option<(Child<'a>, Child<'a>)>
}

impl<'a> Node<'a> {
    fn new(value: i32, children: Option<(Child<'a>, Child<'a>)>) -> Self {
        Node { value, children }
    }

    fn search(&'a self, target: i32) -> Option<&'a Node<'a>> {
        #[cfg(feature = "debug")]
        println!("on {}", self.value);

        // found!
        if self.value == target { return Some(&self); }

        if self.children.is_none() { return None; }

        return match target {
            // differs between left and right children
            _ if target <= self.value => {
                self.children.unwrap().0.search(target)
            }
            _ => {
                self.children.unwrap().1.search(target)
            }
        }
    }
}

fn main() {
    let child5 = Node::new(7, None);
    let child6 = Node::new(10, None);

    let child3 = Node::new(4, None);
    let child4 = Node::new(5, Some((&child5, &child6)));

    let child1 = Node::new(2, None);
    let child2 = Node::new(3, Some((&child3, &child4)));

    let root = Node::new(-1, Some((&child1, &child2)));

    match root.search(3) {
        Some(node) => println!("found! {:?}", node),
        None => println!("not found!")
    };
}
