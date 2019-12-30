use binarytree::node;

fn main() {
    let root_val = 5;
    let mut root = node::Node::new(&root_val);
    root.insert(&3);
    root.insert(&4);
    root.insert(&1);
    root.insert(&6);

    root.display();
}
