use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SelectOption<'a> {
    pub index: usize,
    pub title: &'a str,
}

impl<'a> SelectOption<'a> {
    pub fn new(index: usize, title: &'a str) -> Self {
        SelectOption { index, title }
    }
}

impl fmt::Display for SelectOption<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}
