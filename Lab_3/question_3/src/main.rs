#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        if self.data == data {
            return;
        }

        let target = if data < self.data {
            &mut self.left_child
        } else {
            &mut self.right_child
        };

        match target {
            Some(node) => node.insert_node(data),
            None => {
                let new_node = TreeNode {
                    data,
                    left_child: None,
                    right_child: None,
                };
                let boxed_node = Some(Box::new(new_node));
                *target = boxed_node;
            }
        }
    }
}

fn main() {
    let mut root = TreeNode {
        data: "5",
        left_child: None,
        right_child: None,
    };

    root.insert_node("3");
    root.insert_node("2");
    root.insert_node("4");
    root.insert_node("7");
    root.insert_node("6");
    root.insert_node("8");

    println!("{:#?}", root);
}
