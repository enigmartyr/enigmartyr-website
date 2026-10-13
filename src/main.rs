use dioxus::logger::tracing::Level;
use crate::genetics::*;
use crate::politics::*;
use dioxus::prelude::*;

mod generic;
mod genetics;
mod politics;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/politics/")]
    Politics {},
    #[route("/politics/:region/:topic/:..content_path")]
    PoliticsPage {
        region: String,
        topic: String,
        content_path: Vec<String>
    },
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
    dioxus::logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // <editor-fold desc="Bloom Filter">
        svg {
            width: "0",
            height: "0",
            defs {
                filter {
                    id: "bloom",
                    x: "0",
                    y: "0",
                    width: "100%",
                    height: "100%",
                    color_interpolation_filters: "linearRGB",
                    feColorMatrix {
                        "in": "SourceGraphic",
                        "type": "matrix",
                        values: "1 0 0 0 -0.3
                                 0 1 0 0 -0.3
                                 0 0 1 0 -0.3
                                 0 0 0 1 0",
                        result: "cut-off-source",
                    },
                    feGaussianBlur {
                        "in": "cut-off-source",
                        std_deviation: "1",
                        result: "blurred-glow",
                        edge_mode: "duplicate"
                    },
                    feComponentTransfer {
                        "in": "blurred-glow",
                        result: "boosted-glow",
                        feFuncR { "type": "linear", slope: "2" }
                        feFuncG { "type": "linear", slope: "2" }
                        feFuncB { "type": "linear", slope: "2" }
                    },
                    feMerge {
                        feMergeNode { "in": "boosted-glow" }
                        feMergeNode { "in": "SourceGraphic" }
                    }
                }
            }
        },
        // </editor-fold>
        // <editor-fold desc="Grid Pattern">
        svg {
            width: "0",
            height: "0",
            defs {
                pattern {
                    id: "grid-pattern",
                    width: "84",
                    height: "48",
                    pattern_units: "userSpaceOnUse",
                    path {
                        d: "M0 0h12v6H0V0zm28 8h12v6H28V8zm14-8h12v6H42V0zm14 0h12v6H56V0zm0 8h12v6H56V8zM42 8h12v6H42V8zm0 16h12v6H42v-6zm14-8h12v6H56v-6zm14 0h12v6H70v-6zm0-16h12v6H70V0zM28 32h12v6H28v-6zM14 16h12v6H14v-6zM0 24h12v6H0v-6zm0 8h12v6H0v-6zm14 0h12v6H14v-6zm14 8h12v6H28v-6zm-14 0h12v6H14v-6zm28 0h12v6H42v-6zm14-8h12v6H56v-6zm0-8h12v6H56v-6zm14 8h12v6H70v-6zm0 8h12v6H70v-6zM14 24h12v6H14v-6zm14-8h12v6H28v-6zM14 8h12v6H14V8zM0 8h12v6H0V8z",
                        fill: "var(--site-dimmed-primary-color)",
                        fill_opacity: "0.4",
                        fill_rule: "evenodd"
                    }
                }
            }
        },
        // </editor-fold>
        document::Link { rel: "icon", href: FAVICON },
        document::Link { rel: "stylesheet", href: MAIN_CSS },
        document::Script { src: "/lib/pdfjs/pdf.mjs", "type": "module" }
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
