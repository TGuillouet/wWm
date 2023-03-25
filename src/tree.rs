use std::{cell::RefCell, fmt::Debug};

#[derive(Debug, PartialEq, Clone)]
pub enum TilingDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub value: T,
    pub direction: TilingDirection,
    pub childrens: Vec<RefCell<Box<Node<T>>>>,
}
impl<T> Node<T> {
    pub fn new(value: T, direction: TilingDirection) -> Self {
        Self {
            value,
            direction,
            childrens: Vec::new(),
        }
    }

    pub fn insert(&mut self, new_val: T, direction: TilingDirection) {
        // if self.childrens.len() > 1 {
        //     self.childrens[1].borrow_mut().insert(new_val, direction);
        //     return;
        // }

        self.childrens
            .push(RefCell::new(Box::new(Node::new(new_val, direction))));
    }

    pub fn is_leaf(&self) -> bool {
        return self.childrens.is_empty();
    }
}
