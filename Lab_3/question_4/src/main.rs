#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

impl<T: Ord> Tree<T> {
    fn new() -> Self {
        Tree::Empty
    }

    fn insert(&mut self, data: T) {
        match self {
            Tree::Node { data: node_data, left_child, right_child } => {
                if &data == node_data {
                    return;
                }
                let target_child = if &data < node_data {
                    left_child
                } else {
                    right_child
                };
                target_child.insert(data);
            }
            Tree::Empty => {
                *self = Tree::Node {
                    data,
                    left_child: Box::new(Tree::Empty),
                    right_child: Box::new(Tree::Empty),
                };
            }
        }
    }
}

fn main() {
    let mut root = Tree::new();

    root.insert(5);
    root.insert(3);
    root.insert(2);
    root.insert(4);
    root.insert(7);
    root.insert(6);
    root.insert(8);

    println!("{:#?}", root);
}
