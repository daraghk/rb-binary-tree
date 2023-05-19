mod rbbt;

fn main() {
    let tree_size = 1000;
    let root = rbbt::TreeNode::new(tree_size);
    println!("{:?}", &root.value);
}
