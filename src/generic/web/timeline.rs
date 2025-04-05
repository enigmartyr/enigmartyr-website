use std::ops::Deref;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use dioxus_logger::tracing::debug;
use log::log;

const TIMELINE_CSS: Asset = asset!("/assets/style/timeline.css");
const ZOOM_POSITIVE_ICON: Asset = asset!("/assets/icon/zoom-positive.svg");
const ZOOM_NEGATIVE_ICON: Asset = asset!("/assets/icon/zoom-negative.svg");
const ZOOM_MIN: f32 = 0.5;
const ZOOM_MAX: f32 = 1.0;
const ZOOM_INC: f32 = 0.1;

struct TimelineScrollState {

}

#[component]
pub fn Timeline(t0: u16, tf: u16, #[props(optional)] id: Option<String>) -> Element {
    let t0 = (t0 + 0) / 10;
    let tf = (tf + 9) / 10;
    let timespan = (tf - t0) * 10 + 20;
    let mut is_mounted = use_signal(|| false);
    let mut zoom_scale = use_signal(|| 1.0);
    let mut mouse_down = use_signal(|| Option::<i32>::None);

    use_effect(move || if let Some(_) = mouse_down() {
        document::eval("document.body.classList.add('ew-scrolling')");
    } else {
        document::eval("document.body.classList.remove('ew-scrolling')");
    });

    rsx!{
        link { rel: "stylesheet", href: TIMELINE_CSS },
        div {
            id: id,
            class: if mouse_down().is_some() { "tl-greater tl-scrolling" } else { "tl-greater" },
            div {
                class: if is_mounted() { "tl-proper mounted" } else { "tl-proper" },
                onmounted:   move |_| if !is_mounted() { is_mounted.set(true) },
                onmouseup:   move |_| mouse_down.set(None),
                onmousedown: move |e| mouse_down.set(Some(e.as_web_event().client_x())),
                onmousemove: move |e| if let Some(x) = mouse_down() {
                    e.stop_propagation();
                    let d = e.as_web_event().client_x();
                    let e = "document.getElementsByClassName('tl-scrolling')[0]";
                    document::eval(&format!("{}.scrollBy({}, 0)", e, x - d));
                    mouse_down.set(Some(d));
                },
                div {
                    class: "tl-padding tl-tail-padding"
                },
                div {
                    class: "tl-button-greater",
                    onmousedown: move |e| e.stop_propagation(),
                    div {
                        class: if zoom_scale() < ZOOM_MAX {
                            "tl-button tl-zoom-in"
                        } else {
                            "tl-button tl-zoom-in disabled"
                        },
                        style: "mask-image: url({ZOOM_POSITIVE_ICON})",
                        onclick: move |_| zoom_scale.set((zoom_scale + ZOOM_INC).min(ZOOM_MAX))
                    },
                    div {
                        class: if zoom_scale() > ZOOM_MIN {
                            "tl-button tl-zoom-out"
                        } else {
                            "tl-button tl-zoom-out disabled"
                        },
                        style: "mask-image: url({ZOOM_NEGATIVE_ICON})",
                        onclick: move |_| zoom_scale.set((zoom_scale - ZOOM_INC).max(ZOOM_MIN))
                    }
                },
                div {
                    class: "tl-segment tl-tail",
                    div {
                        class: "tl-line-segment"
                    }
                },
                div {
                    style: "--length: {timespan}",
                    class: "tl-segment tl-body",
                    div {
                        class: "tl-line-segment"
                    },
                    for i in 1..timespan {
                        div {
                            style: "--offset: {i}",
                            class: match (i % 5, i % 10) {
                                (0, 0) => "tl-tick tl-large-tick",
                                (0, _) => "tl-tick tl-small-tick",
                                (_, _) => "tl-tick tl-tiny-tick"
                            },
                            "data-date": "{t0 * 10 + i - 10}"
                        }
                    }
                },
                div {
                    class: "tl-segment tl-head",
                    div {
                        class: "tl-line-segment"
                    }
                },
                div {
                  class: "tl-padding tl-head-padding"
                }
            }
        }
    }
}