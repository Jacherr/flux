use std::io;
use std::ops::{Deref, DerefMut};
use std::process::{Child, Output};

pub struct OwnedChild(Option<Child>);

impl OwnedChild {
    pub fn new(c: Child) -> Self {
        Self(Some(c))
    }

    pub fn wait_with_output(mut self) -> io::Result<Output> {
        self.0.take().unwrap().wait_with_output()
    }
}

impl Deref for OwnedChild {
    type Target = Child;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for OwnedChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for OwnedChild {
    fn drop(&mut self) {
        // If it's None, it must have been moved out in the wait_with_output function
        // in which case it doesn't need to be reaped

        if let Some(this) = &mut self.0 {
            let _ = this.kill();
            let _ = this.wait();
        }
    }
}

pub trait IntoOwnedChild {
    fn into_owned_child(self) -> OwnedChild;
}

impl IntoOwnedChild for Child {
    fn into_owned_child(self) -> OwnedChild {
        OwnedChild::new(self)
    }
}
