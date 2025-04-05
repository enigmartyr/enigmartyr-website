use dioxus::prelude::*;

#[derive(PartialEq)]
pub struct File {
    pub record: Asset,
    pub banner: &'static str,
    pub source: &'static str,
    pub credit: &'static str
}