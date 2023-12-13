use std::fmt::{Debug, Display};

#[derive(Clone,Eq, Hash, PartialEq)]
pub struct StringTablePath {
    pub(crate) path: Box<[Box<str>]>,
}

impl Display for StringTablePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for part in self.path.iter() {
            path.push_str(part);
            path.push_str("::");
        }
        write!(f, "{}", path)
    }
}

impl Debug for StringTablePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<StringTablePath> for &str {
    fn into(self) -> StringTablePath {
        let split = self.split("::").map(|s| s.into()).collect::<Vec<Box<str>>>();
        for part in split.iter() {
            if part.is_empty() {
                return StringTablePath {
                    path: Box::new([]),
                }
            }
        }
        StringTablePath {
            path: split.into_boxed_slice()
        }

    }
}
impl Into<StringTablePath> for Vec<Box<str>> {
    fn into(self) -> StringTablePath {
        for part in self.iter() {
            if part.is_empty() {
                return StringTablePath {
                    path: Box::new([]),
                }
            }
        }
        StringTablePath {
            path: self.into_boxed_slice()
        }
    }
}

#[derive(Clone)]
pub struct FunctionPath {
    pub(crate) path: Box<[Box<str>]>
}


impl Display for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for (i, part) in self.path.iter().enumerate() {
            path.push_str(part);
            if i != self.path.len() - 1 {
                path.push_str("::");
            }
        }
        write!(f, "{}", path)
    }
}
impl Debug for FunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<FunctionPath> for &str {
    fn into(self) -> FunctionPath {
        FunctionPath {
            path: self.split("::").map(|s| s.into()).collect::<Vec<Box<str>>>().into_boxed_slice()
        }
    }
}