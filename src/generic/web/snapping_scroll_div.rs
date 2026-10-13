use dioxus::prelude::*;
use dioxus::html::Key;

// Note: Removed unused import `use crate::generic::web::typed_div::TypedDivProps;`
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;

// ============================================================================
// Component Properties
// ============================================================================

#[derive(Props, Clone, PartialEq)]
pub struct SnappingScrollDivProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Optional integer number of lines to skip.
    #[props(default = 1u32)]
    skip: u32,
    /// The inner elements that will be scrolled over.
    children: Element,
}

// ============================================================================
// Main Component
// ============================================================================

#[component]
pub fn SnappingScrollDiv(mut props: SnappingScrollDivProps) -> Element {
    // Tracks the actual DOM node so we can read its computed styles and manipulate its scroll state.
    let mut mounted_target = use_signal(|| Option::<std::rc::Rc<MountedData>>::None);

    // Tracks resize events to recalculate the line height if the font size changes.
    let mut element_resize = use_signal(|| Option::<std::rc::Rc<ResizeData>>::None);

    // The calculated height of a single line of text in pixels. Defaults to 1 to avoid division by zero.
    let mut line_height_px = use_signal(|| 1_f64);

    // The absolute target Y coordinate for the scroll position.
    let mut scroll_y_coord = use_signal(|| 0_f64);

    // ------------------------------------------------------------------------
    // Effect 1: Calculate Grid Size (Line Height)
    // ------------------------------------------------------------------------
    #[cfg(feature = "web")]
    use_effect(move || {
        // We recalculate whenever the element is resized (which also fires once on initial mount).
        if let Some(_) = element_resize() {
            if let Some(element) = mounted_target().map(|m| m.as_web_event()) {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(computed_style)) = window.get_computed_style(&element) {

                        // Helper closure to extract numeric pixel values from CSS strings (e.g., "16px" -> 16.0)
                        let get_px_value = |property: &str| -> f64 {
                            computed_style
                                .get_property_value(property)
                                .ok()
                                .and_then(|val| val.trim_end_matches("px").parse::<f64>().ok())
                                .unwrap_or(0.0)
                        };

                        let parsed_line_height = get_px_value("line-height");

                        // Fallback: If `line-height` is missing or set to "normal", approximate it as 1.2x the font size.
                        let final_line_height = if parsed_line_height > 0.0 {
                            parsed_line_height
                        } else {
                            get_px_value("font-size") * 1.2
                        };

                        // Ensure we never return a line height of 0.
                        line_height_px.set(final_line_height.max(1.0));
                    }
                }
            }
        }
    });

    // ------------------------------------------------------------------------
    // Effect 2: Enforce Scroll Snapping on State Change
    // ------------------------------------------------------------------------
    #[cfg(feature = "web")]
    use_effect(move || if let Some(element) = mounted_target().map(|m| m.as_web_event()) {
        element.set_scroll_top(((scroll_y_coord() / line_height_px()).round() * line_height_px()) as i32);
    });

    // ------------------------------------------------------------------------
    // Render & Event Overrides
    // ------------------------------------------------------------------------
    rsx! {
        div {
            class: "snapping-scroll",
            tabindex: "0", // Required to make the div focusable for keyboard events

            // --- Initialization Observers ---
            onmounted: move |event: MountedEvent| mounted_target.set(Some(event.data())),
            onresize: move |event: ResizeEvent| element_resize.set(Some(event.data())),

            // --- Mouse Wheel Override ---
            onwheel: move |event: WheelEvent| {
                event.prevent_default(); // Disable native smooth scroll

                #[cfg(feature = "web")]
                if let Some(element) = mounted_target().map(|m| m.as_web_event()) {
                    let current_y = scroll_y_coord();
                    let wheel_delta = event.data().delta().strip_units().y;
                    let max_scroll = (element.scroll_height() - element.client_height()).max(0);

                    // Apply the delta and clamp strictly to the boundaries of the container
                    let new_y = (current_y + wheel_delta).clamp(0f64, max_scroll as f64);
                    scroll_y_coord.set(new_y);
                }
            },

            // --- Keyboard Navigation Override ---
            onkeydown: move |event: KeyboardEvent| {
                #[cfg(feature = "web")]
                if let Some(element) = mounted_target().map(|m| m.as_web_event()) {

                    // Map specific keys to line multipliers (e.g., PageDown moves 5 lines at a time)
                    let line_multiplier = match event.key() {
                        Key::ArrowUp => Some(-1),
                        Key::ArrowDown => Some(1),
                        Key::PageUp => Some(-5),
                        Key::PageDown => Some(5),
                        Key::Home => Some(-i32::MAX), // Sufficiently large to hit absolute top
                        Key::End => Some(i32::MAX),   // Sufficiently large to hit absolute bottom
                        _ => None
                    };

                    if let Some(multiplier) = line_multiplier {
                        let current_y = scroll_y_coord();
                        let grid_size = line_height_px();
                        let max_scroll = (element.scroll_height() - element.client_height()).max(0) as f64;

                        // Calculate translation amount and clamp to container bounds
                        let move_amount = grid_size * multiplier as f64;
                        let new_y = (current_y + move_amount).clamp(0f64, max_scroll);

                        scroll_y_coord.set(new_y);
                        event.prevent_default();
                    }
                }
            },

            // --- Scroller / Text Selection Override ---
            onscroll: move |event: ScrollEvent| {
                #[cfg(feature = "web")]
                if let Some(element) = mounted_target().map(|m| m.as_web_event()) {
                    let actual_top = element.scroll_top() as f64;
                    let grid_size = line_height_px();

                    // 1. Calculate the nearest "snapped" line
                    let snapped_y = (actual_top / grid_size).round() * grid_size;

                    // 2. Check if we are currently "off-grid"
                    // If the browser moved us to 12.5px but our grid is 20px,
                    // this will trigger and jump us to 20px (or 0px).
                    if (actual_top - snapped_y).abs() > 0.1 {
                        // Sync the signal so other parts of the app know where we are
                        scroll_y_coord.set(snapped_y);

                        // Force the DOM element to the snapped position immediately
                        element.set_scroll_top(snapped_y as i32);
                    }
                }
            },

            ..props.attributes,
            { props.children }
        }
    }
}