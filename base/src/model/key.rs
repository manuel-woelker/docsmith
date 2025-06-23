use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

pub enum Key {
    //    U64(u64),
    String(Arc<String>),
    Str(&'static str),
}

impl From<&'static str> for Key {
    fn from(value: &'static str) -> Self {
        Key::Str(value)
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key::String(Arc::new(value))
    }
}

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Key {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Deref for Key {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Key::String(s) => s.as_str(),
            Key::Str(s) => s,
        }
    }
}

impl PartialEq<Key> for Key {
    fn eq(&self, key: &Key) -> bool {
        self.as_str() == key.as_str()
    }
}

impl Eq for Key {}
