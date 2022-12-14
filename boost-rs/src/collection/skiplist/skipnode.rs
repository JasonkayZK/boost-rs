use std::iter;
use std::ptr::NonNull;

/// A covariant pointer to a SkipNode.
///
/// SkipNode<V> should contain mutable pointers to other nodes,
/// but mutable pointers are not covariant in Rust.
/// The appropriate pointer type is std::ptr::NonNull.
///
/// See [`NonNull`] and Rustonomicon for details on covariance.
/// https://doc.rust-lang.org/nomicon/subtyping.html
pub(crate) type Link<T> = Option<NonNull<SkipNode<T>>>;

/// SkipNodes are make up the SkipList.  The SkipList owns the first head-node
/// (which has no value) and each node has ownership of the next node through
/// `next`.
///
/// The node has a `level` which corresponds to how 'high' the node reaches.
///
/// A node of `level` n has (n + 1) links to next nodes, which are stored in
/// a vector.
///
/// The node linked by level 0 should be considered owned by this node.
///
/// There is a corresponding vector of link lengths which contains the distance
/// between current node and the next node. If there's no next node, the distance
/// is distance between current node and last reachable node.
///
/// Lastly, each node contains a link to the immediately previous node in case
/// one needs to parse the list backwards.
#[derive(Clone, Debug)]
pub(crate) struct SkipNode<T> {
    // item should never be None, unless the node is a head.
    pub(crate) val: Option<T>,

    // how high the current node reaches.
    pub(crate) level: usize,

    // Vector of links to the next node at the respective level. This vector
    // *must* be of length `self.level + 1`.  links[0] stores a pointer to the
    // next node, which will have to be dropped.
    pub(crate) links: Vec<Link<T>>,
}

impl<T> SkipNode<T> {
    /// Create a new head node.
    pub fn head(level_bound: usize) -> Self {
        SkipNode {
            val: None,
            level: level_bound - 1, // The head node has `level_bound-1` levels(highest level)
            links: iter::repeat(None).take(level_bound).collect(),
        }
    }

    /// Create a new SkipNode with the given item..
    /// All pointers default to null.
    pub fn new(item: T, level: usize) -> Self {
        SkipNode {
            val: Some(item),
            level,
            links: iter::repeat(None).take(level + 1).collect(),
        }
    }

    pub fn into_val(self) -> Option<T> {
        self.val
    }
}
