use dioxus::prelude::*;

pub mod tw;
pub mod data;

#[component]
pub fn Politics() -> Element {
    rsx! {
        div {
            id: "politics"
        }
    }
}