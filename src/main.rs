extern crate alloc;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use crate::politics::*;
use crate::politics::tw::*;
use crate::politics::tw::sovereignty::*;
use crate::genetics::*;

mod politics;
mod genetics;
mod generic;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/politics/")]
    Politics {},
    #[route("/politics/tw/")]
    PoliticsTaiwan {},
    #[route("/politics/tw/sovereignty/")]
    PoliticsTaiwanSovereignty {},
    #[route("/genetics/")]
    Genetics {},
    //#[route("/blog/:id")]
    //Blog { id: i32 },

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/style/main.css");

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            },
            Link {
                to: Route::Politics {},
                "Politics"
            },
            Link {
                to: Route::Genetics {},
                "Genetics"
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            id: "home"
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}