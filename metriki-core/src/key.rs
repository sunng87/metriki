use std::cmp;
use std::hash::{Hash, Hasher};
use serde::Serialize;

pub type KeyName = String;

#[derive(Debug, Clone, Serialize)]
pub struct Key {
    pub(crate) name: KeyName,
    pub(crate) tags: Vec<Tag>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord, Serialize)]
pub struct Tag {
    key: String,
    value: String,
}

impl Key {
    pub fn from_name(name: &str) -> Self {
        Key {
            name: name.to_owned(),
            tags: Vec::with_capacity(0),
        }
    }

    pub fn from(name: &str, tags: Vec<Tag>) -> Self {
        Key {
            name: name.to_owned(),
            tags,
        }
    }

    pub fn key(&self) -> &str {
        self.name.as_str()
    }

    pub fn tags(&self) -> &[Tag] {
        self.tags.as_slice()
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.tags == other.tags
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (&self.name, &self.tags).cmp(&(&other.name, &other.tags))
    }
}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        key_hasher_impl(state, &self.name, &self.tags);
    }
}

impl Tag {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

fn key_hasher_impl<H: Hasher>(state: &mut H, name: &KeyName, tags: &[Tag]) {
    name.hash(state);
    tags.hash(state);
}
