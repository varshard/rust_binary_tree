pub mod node {
    use std::fmt::Display;

    type Branch<'a, T> = Option<Box<Node<'a, T>>>;

    pub struct Node<'a, T: PartialOrd + Display> {
        value: &'a T,
        left: Branch<'a, T>,
        right: Branch<'a, T>,
        is_right: bool,
    }

    impl<'a, T: PartialOrd + Display> Node<'a, T> {
        pub fn new(value: &'a T) -> Node<'a, T> {
            Node {
                value: value,
                left: None,
                right: None,
                is_right: false,
            }
        }
        pub fn insert(&mut self, value: &'a T) {
            let target_node = if value > self.value {
                &mut self.right
            } else {
                &mut self.left
            };
            match target_node {
                Some(ref mut node) => node.insert(value),
                None => {
                    let mut new_node = Node::new(value);
                    new_node.is_right = value > self.value;
                    *target_node = Some(Box::new(new_node));
                }
            }
        }
        pub fn display(&'a self) {
            let root = vec![Some(self)];
            let mut children: Vec<Option<&Node<T>>> = self.get_children(&root);

            println!("{}", self.value);

            while children.len() > 0 {
                for child in &children {
                    match child {
                        Some(node) => {
                            if node.is_right {
                                print!("\\");
                            } else {
                                print!("|");
                            }
                        }
                        None => {
                            print!(" ");
                        }
                    }
                }
                println!();
                for child in &children {
                    match child {
                        Some(node) => {
                            print!("{} ", node.value);
                        }
                        None => {
                            print!(" ");
                        }
                    }
                }
                println!();
                children = self.get_children(&children);
            }
        }
        fn get_children<'b>(
            &self,
            nodes: &Vec<Option<&'b Node<'a, T>>>,
        ) -> Vec<Option<&'b Node<'a, T>>> {
            let mut children: Vec<Option<&Node<'a, T>>> = Vec::new();
            for node_option in nodes {
                match node_option {
                    Some(node) => match &node.left {
                        Some(left) => {
                            children.push(Some(left));
                            match &node.right {
                                Some(right) => {
                                    children.push(Some(right));
                                }
                                None => {
                                    children.push(None);
                                }
                            }
                        }
                        None => {
                            children.push(None);
                            match &node.right {
                                Some(right) => {
                                    children.push(Some(right));
                                }
                                None => {
                                    children.push(None);
                                }
                            }
                        }
                    },
                    None => {}
                }
            }

            children
        }
    }
}
