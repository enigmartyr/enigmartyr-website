use dioxus::prelude::*;

pub mod sovereignty;

#[component]
pub fn PoliticsTaiwan() -> Element {
    rsx! {
        div {
            id: "taiwan"
        }
    }
}