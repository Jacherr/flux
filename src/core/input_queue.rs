use std::cell::RefCell;
use std::rc::Rc;

/// The input queue
pub struct InputQueue(Rc<RefCell<Vec<Vec<u8>>>>);
impl InputQueue {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }

    pub fn push(&self, input: Vec<u8>) {
        self.0.borrow_mut().push(input);
    }

    pub fn unshift(&self) -> Option<Vec<u8>> {
        if self.len() > 0 {
            Some(self.0.borrow_mut().remove(0))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
}
