use std::fmt::Display;
use std::fs;

#[derive(Clone)]
pub struct TmpFile(String, String);
impl TmpFile {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str> + Display,
    {
        TmpFile(format!("/tmp/{}-flux-{name}", std::process::id()), name.to_string())
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

    pub fn filename(&self) -> &str {
        &self.1
    }
}
impl Drop for TmpFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

#[derive(Clone)]
pub struct TmpFolder(String, String);
impl TmpFolder {
    pub fn new<S>(name: S) -> Self
    where
        S: AsRef<str> + Display,
    {
        let path = format!("/tmp/{}-flux-{name}", std::process::id());
        TmpFolder(path, name.to_string())
    }

    pub fn path(&self) -> &str {
        &self.0
    }

    pub fn filename(&self) -> &str {
        &self.1
    }
}
impl Drop for TmpFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
