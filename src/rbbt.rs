use num::{Integer, One, Zero};
use rand::Rng;

pub struct TreeNode<N> {
    pub value: Option<N>,
    lower_bound: N,
    upper_bound: N,
    pub left: Option<Box<TreeNode<N>>>,
    pub right: Option<Box<TreeNode<N>>>,
    height: Option<usize>,
}

impl<
        N: Integer
            + num::Signed
            + rand::distributions::uniform::SampleUniform
            + std::fmt::Debug
            + Copy
            + num::FromPrimitive,
    > TreeNode<N>
{
    pub fn new(tree_size: N) -> Box<Self> {
        let random_value = rand::thread_rng().gen_range(Zero::zero()..tree_size);
        let mut root = Box::new(TreeNode {
            value: Some(random_value),
            lower_bound: Zero::zero(),
            upper_bound: tree_size,
            left: None,
            right: None,
            height: Some(1)
        });
        Self::add_child_nodes(&mut root);
        root
    }

    fn add_child_nodes(parent: &mut Box<TreeNode<N>>) {
        let parent_lower_bound = parent.lower_bound;
        let parent_upper_bound = parent.upper_bound;
        let parent_value = parent.value.unwrap();

        let left_child = Self::create_left_child(parent_lower_bound, parent_value);
        let right_child = Self::create_right_child(parent_value, parent_upper_bound);

        let null_marker: N = num::cast::FromPrimitive::from_i32(-1).unwrap();
        println!(
            "root {:?} left {:?} right {:?}",
            parent_value,
            left_child.value.unwrap_or_else(|| null_marker),
            right_child.value.unwrap_or_else(|| null_marker)
        );

        if left_child.value.is_some() {
            parent.left = Some(Box::new(left_child));
            Self::add_child_nodes(parent.left.as_mut().unwrap());
        }

        if right_child.value.is_some() {
            parent.right = Some(Box::new(right_child));
            Self::add_child_nodes(parent.right.as_mut().unwrap());
        }
    }

    fn create_left_child(parent_lower_bound: N, parent_value: N) -> Self {
        let mut left_child = TreeNode {
            value: None,
            lower_bound: parent_lower_bound,
            upper_bound: parent_value,
            left: None,
            right: None,
            height: None
        };
        let left_child_range_len = num::abs(left_child.upper_bound - left_child.lower_bound);
        if !left_child_range_len.is_zero() {
            let left_child_value =
                rand::thread_rng().gen_range(left_child.lower_bound..left_child.upper_bound);
            left_child.value = Some(left_child_value);
        }
        left_child
    }

    fn create_right_child(parent_value: N, parent_upper_bound: N) -> Self {
        let mut right_child = TreeNode {
            value: None,
            lower_bound: parent_value + One::one(),
            upper_bound: parent_upper_bound,
            left: None,
            right: None,
            height: None
        };
        let right_child_range_len = num::abs(right_child.upper_bound - right_child.lower_bound);
        if !right_child_range_len.is_zero() {
            let right_child_value =
                rand::thread_rng().gen_range(right_child.lower_bound..right_child.upper_bound);
            right_child.value = Some(right_child_value);
        }
        right_child
    }
}
