use dioxus::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, PartialEq)]
pub struct File {
    pub internal_source: Asset,
    pub external_source: &'static str,
    pub source_descript: &'static str,
    pub source_citation: &'static str,
}

impl Hash for File {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal_source.resolve().hash(state);
    }
}

impl File {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
