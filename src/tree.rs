#[derive(Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub childrens: Vec<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            childrens: Vec::new(),
        }
    }

    pub fn insert(&mut self, new_val: T) {
        self.childrens.push(Box::new(Node::new(new_val)))
    }

    pub fn is_leaf(&self) -> bool {
        return self.childrens.is_empty();
    }
}
