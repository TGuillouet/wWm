use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum TilingDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
    pub direction: TilingDirection,
    pub childrens: Vec<Box<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(value: T, direction: TilingDirection) -> Self {
        Self {
            value,
            direction,
            childrens: Vec::new(),
        }
    }

    pub fn insert(&mut self, new_val: T, index: usize, direction: TilingDirection) {
        self.childrens
            .insert(index, Box::new(Node::new(new_val, direction)));
    }

    pub fn is_leaf(&self) -> bool {
        return self.childrens.is_empty();
    }
}
