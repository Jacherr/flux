use std::cell::RefCell;
use std::rc::Rc;

use crate::processing::media_object::MediaObject;

/// The input queue. Inputs can be of several types depending on the source. This can help
/// unnecessary re-encodes when chaining operations.
pub struct InputQueue(Rc<RefCell<Vec<MediaObject>>>);
impl InputQueue {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }

    pub fn push(&self, input: MediaObject) {
        self.0.borrow_mut().push(input);
    }

    pub fn unshift(&self) -> Option<MediaObject> {
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
