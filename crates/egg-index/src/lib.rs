#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Node<T> {
    data: Vec<T>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Tree<T> {
    root: Node<T>,
    tail: Option<Vec<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = vec![String::from("Hello"), String::from("Egg")];
        let root = Node { data };
        let expected = Tree {
            root: root.clone(),
            tail: None,
        };
        let actual = Tree {
            root,
            tail: Some(Vec::new()),
        };
        assert_eq!(expected, actual);
    }
}
