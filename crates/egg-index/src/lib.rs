// https://eugene-babichenko.github.io/blog/2019/11/13/rust-popcount-intrinsics/
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
fn popcnt(n: u64) -> u32 {
    n.count_ones()
}

const ALPHABET_SIZE: u64 = 64;

fn is_set(bitmap: u64, n: u64) -> bool {
    let shift = ALPHABET_SIZE - n - 1;
    let mask = 1 << shift; // Shift 0b1 n places to the left
    bitmap & mask != 0
}

fn bitmap_index_of(bitmap: u64, key: u64) -> usize {
    let shift = ALPHABET_SIZE - key - 1;
    (popcnt(bitmap >> shift) - 1) as usize
}

#[cfg(test)]
mod bitmap_tests {
    use super::*;

    #[test]
    fn is_set_test() {
        let bitmap = 0b10100000 << (ALPHABET_SIZE - 8);
        assert!(is_set(bitmap, 0));
        assert!(!is_set(bitmap, 1));
        assert!(is_set(bitmap, 2));
        assert!(!is_set(bitmap, 3));
    }

    #[test]
    fn bitmap_index_of_test() {
        let bitmap = 0b01011000 << (ALPHABET_SIZE - 8);
        assert_eq!(0, bitmap_index_of(bitmap, 1));
        assert_eq!(1, bitmap_index_of(bitmap, 3));
        assert_eq!(2, bitmap_index_of(bitmap, 4));
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Tree<T> {
    bitmap: u64,
    children: Vec<Node<T>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Leaf<T> {
    data: Vec<T>,
}

impl<T> Leaf<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Node<T> {
    Leaf(Leaf<T>),
    Tree(Tree<T>),
}

impl<T> Tree<T> {
    pub fn new(bitmap: u64, children: Vec<Node<T>>) -> Self {
        Self { bitmap, children }
    }

    pub fn entry_for(&self, key: u64) -> Option<&Node<T>> {
        if is_set(self.bitmap, key) {
            println!("It's set: {:b}", key);
            let index = bitmap_index_of(self.bitmap, key);
            self.children.get(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tree_tests {
    use super::*;

    #[test]
    fn basic_test() {
        let node1 = Node::Leaf(Leaf::new(vec!["foo", "bar"]));
        let node2 = Node::Leaf(Leaf::new(vec!["baz"]));

        let tree = Tree::new(
            0b10100000 << (ALPHABET_SIZE - 8),
            vec![node1.clone(), node2.clone()],
        );

        assert_eq!(Some(&node1), tree.entry_for(0));
        assert_eq!(None, tree.entry_for(1));
        assert_eq!(Some(&node2), tree.entry_for(2));
    }

    #[test]
    fn nested_test() {
        let node1 = Node::Leaf(Leaf::new(vec!["foo", "bar"]));
        let node2 = Node::Leaf(Leaf::new(vec!["baz"]));
        let node3 = Node::Tree(Tree::new(
            0b10100000 << (ALPHABET_SIZE - 8),
            vec![node1.clone(), node2.clone()],
        ));

        let tree = Tree::new(0b10000000 << (ALPHABET_SIZE - 8), vec![node3.clone()]);

        let t1 = tree.entry_for(0);
        match t1 {
            Some(Node::Tree(t)) => {
                assert_eq!(t.entry_for(0), Some(&node1));
                assert_eq!(t.entry_for(2), Some(&node2));
            }
            _ => todo!(),
        }
    }
}
