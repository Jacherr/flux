use std::fmt::Display;
use std::fs;

#[derive(Clone)]
pub struct TmpFile(String);
impl TmpFile {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str> + Display,
    {
        TmpFile(format!("/tmp/{}-flux-{name}", std::process::id()))
    }

    pub fn write<C>(&self, contents: C) -> std::io::Result<()>
    where
        C: AsRef<[u8]>,
    {
        fs::write(&self.0, contents)
    }

    pub fn path(&self) -> &str {
        &self.0
    }
}
impl Drop for TmpFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
