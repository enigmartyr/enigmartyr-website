use dioxus::prelude::*;
use std::{cell::RefCell, rc::{Rc, Weak}};

#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use wasm_bindgen::{prelude::*, JsCast};
// ============================================================================
// Constants & Helper Functions
// ============================================================================

/// Determines if a character falls within common Chinese, Japanese, or Korean (CJK) Unicode blocks.
/// CJK characters are visually denser and often take longer to read/type, so we assign them a higher typing "cost".
fn is_cjk(c: char) -> bool {
    matches!(c, '\u{2E80}'..='\u{2EFF}' | '\u{3000}'..='\u{9FFF}' | '\u{AC00}'..='\u{D7A3}')
}

// ============================================================================
// Data Structures
// ============================================================================

/// Represents a single character and the time "cost" required to type it.
#[derive(Clone, Copy, Debug)]
struct CharData {
    /// The actual character to be typed.
    char: char,
    /// The time cost of typing this character (e.g., 1.0 for standard ASCII, 10.0 for CJK).
    cost: f64,
    /// Flag indicating if this is a CJK character, used to trigger custom cursor animations.
    is_cjk: bool,
}

/// Represents a distinct DOM text node that is being typed out.
#[cfg(feature = "web")]
struct TextSpan {
    /// The actual DOM Text Node that will have characters appended to it over time.
    node: web_sys::Text,
    /// The sequence of characters (and their metadata) that belong to this node.
    data: Vec<CharData>,
    /// An optional event trigger string parsed from the DOM (`data-trigger` attribute).
    trigger: Option<String>,
}

/// Tracks the mathematical, positional, and cached DOM progress of the typing animation.
#[cfg(feature = "web")]
struct AnimationProgressState {
    // --- Time & Cost Tracking ---
    /// The timestamp (in milliseconds) when the animation started. -1.0 means uninitialized.
    start_time: f64,
    /// The accumulated time cost of characters already typed in the *current* span.
    accumulated_cost_in_span: f64,
    /// The total time cost of all fully completed spans up to this point.
    total_cost_processed: f64,

    // --- Positional Indices ---
    /// The index of the `TextSpan` currently being typed.
    span_index: usize,
    /// The index of the character currently being typed within the active `TextSpan`.
    char_index: usize,
    /// Tracks which span the cursor DOM element is currently inserted next to.
    current_cursor_span_index: Option<usize>,

    // --- DOM Caching (Performance) ---
    /// Cache of the cursor's CSS class to prevent redundant WASM boundary crossings.
    last_cursor_class: &'static str,
    /// Cache of the cursor's text content to prevent redundant WASM boundary crossings.
    last_cursor_text: String,
}

/// Holds the state of the active `requestAnimationFrame` (RAF) loop.
/// This prevents memory leaks and allows us to cancel the animation on reset or unmount.
#[cfg(feature = "web")]
struct AnimationLoopTracker {
    /// The ID returned by `requestAnimationFrame`, used to cancel it.
    request_id: Option<i32>,
    /// The active JavaScript closure executing the animation loop.
    _callback: Option<Closure<dyn FnMut()>>,
}

#[cfg(feature = "web")]
impl Drop for AnimationLoopTracker {
    fn drop(&mut self) {
        // Automatically clean up the animation frame when the component unmounts or tracker is dropped.
        if let Some(id) = self.request_id {
            let _ = web_sys::window().unwrap().cancel_animation_frame(id);
        }
    }
}

// ============================================================================
// Component Properties
// ============================================================================

#[derive(Props, Clone)]
pub struct TypedDivProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    #[props(into, default = "div".to_string())]
    tag: String,
    /// The inner Dioxus elements to be typed out.
    children: Element,

    // --- Configuration ---
    /// The base delay (in milliseconds) per standard character cost.
    #[props(default = Signal::new(2.0))]
    delay_ms: Signal<f64>,

    // --- Control Signals (Inputs) ---
    /// Signal to begin the typing animation.
    #[props(default = Signal::new(true))]
    initiate: Signal<bool>,
    /// Signal to interrupt, clear the text, and reset the animation state.
    #[props(default = Signal::new(false))]
    do_reset: Signal<bool>,

    // --- Output Signals & Events ---
    /// Signal emitted as `true` when the entire text has finished typing.
    #[props(default = Signal::new(false))]
    finished: Signal<bool>,
    /// Optional callback triggered when a boundary with a `data-trigger` attribute is crossed.
    #[props(optional)]
    on_event: Option<EventHandler<String>>,
}

impl PartialEq for TypedDivProps {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes && self.tag == other.tag
    }
}

macro_rules! generate_typed_component {
    ($name:ident, $tag:ident) => {
        #[component]
        pub fn $name(mut props: TypedDivProps) -> Element {
            // --- Internal State (Same as your original code) ---
            let mut mounted_target = use_signal(|| Option::<std::rc::Rc<MountedData>>::None);
            let mut extracted_text = use_signal(String::new);
            let mut dom_was_parsed = use_signal(|| false);
            let mut is_now_visible = use_signal(|| false);
            let mut is_now_running = use_signal(|| false);

            #[cfg(feature = "web")]
            let mut text_node_list = use_signal(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<TextSpan>::new())));
            #[cfg(feature = "web")]
            let mut cursor_element = use_signal(|| Option::<web_sys::Element>::None);
            #[cfg(feature = "web")]
            let animation_loop = use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(AnimationLoopTracker { request_id: None, _callback: None })));

             // ------------------------------------------------------------------------
            // Effect 1: DOM Initialization & Parsing
            // ------------------------------------------------------------------------
            // Only run once, and only when the root element is successfully mounted
            #[cfg(feature = "web")]
            use_effect(move || if !dom_was_parsed() && let Some(root_event_data) = mounted_target().map(|e| e.as_web_event()) {
                dom_was_parsed.set(true);

                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();

                // Save the full text for screen reader accessibility
                if let Some(text) = root_event_data.text_content() {
                    extracted_text.set(text);
                }

                // Create the blinking cursor element
                let cursor = document.create_element("span").unwrap();
                cursor.set_class_name("typing-cursor blinking");
                let _ = cursor.set_attribute("aria-hidden", "true"); // Hide cursor from screen readers
                cursor_element.set(Some(cursor));

                // Prepare to walk the DOM to find all text nodes inside this container
                let tree_walker = document.create_tree_walker_with_what_to_show(&root_event_data, 4).unwrap(); // 4 = NodeFilter.SHOW_TEXT
                let mut text_nodes = vec![];

                while let Ok(Some(node)) = tree_walker.next_node() {
                    // Ensure we don't accidentally parse text from nested `TypedDiv` components
                    if let Some(parent) = node.parent_element() {
                        if let Ok(Some(closest_container)) = parent.closest("[data-typed-container]") {
                            if closest_container != root_event_data {
                                continue;
                            }
                        }
                    }
                    text_nodes.push(node);
                }

                // Process each text node: extract text, calculate costs, and replace with empty DOM nodes
                let mut parsed_spans = Vec::with_capacity(text_nodes.len());

                for node in text_nodes {
                    let text_content = node.text_content().unwrap_or_default();
                    if text_content.is_empty() { continue; }

                    // Look for a custom `data-trigger` attribute on the parent
                    let trigger_event_name = node.parent_element()
                        .and_then(|el| el.closest("[data-trigger]").ok().flatten())
                        .and_then(|el| el.get_attribute("data-trigger"));

                    // Map characters to their animation costs
                    let char_data = text_content.chars().map(|character| {
                        let is_c = is_cjk(character);
                        CharData {
                            char: character,
                            cost: if is_c { 6.0 } else { 1.0 },
                            is_cjk: is_c
                        }
                    }).collect();

                    // Create a blank text node to replace the existing populated one in the DOM
                    let blank_node = document.create_text_node("");
                    let _ = node.dyn_into::<web_sys::CharacterData>().unwrap().replace_with_with_node_1(&blank_node);

                    parsed_spans.push(TextSpan { node: blank_node, data: char_data, trigger: trigger_event_name });
                }

                // Edge Case: If there's no text to type at all, mark as finished immediately
                if parsed_spans.is_empty() {
                    props.finished.set(true);
                }

                *text_node_list.write() = Rc::new(RefCell::new(parsed_spans));
            });

            // ------------------------------------------------------------------------
            // Effect 2: Manual Reset Handler
            // ------------------------------------------------------------------------
            #[cfg(feature = "web")]
            let loop_for_reset = animation_loop.clone();

            #[cfg(feature = "web")]
            use_effect(move || {
                if (props.do_reset)() {
                    // 1. Halt any ongoing animation
                    let mut tracker_lock = loop_for_reset.borrow_mut();
                    if let Some(id) = tracker_lock.request_id {
                        let _ = web_sys::window().unwrap().cancel_animation_frame(id);
                        tracker_lock.request_id = None;
                    }

                    // 2. Erase the text from all tracked DOM nodes
                    let spans_reference = text_node_list.read().clone();
                    for span in spans_reference.borrow().iter() {
                        span.node.set_data("");
                    }

                    // 3. Reset and hide the cursor
                    if let Some(cursor) = cursor_element.read().as_ref() {
                        cursor.set_text_content(None);
                        cursor.set_class_name("typing-cursor blinking");
                        cursor.remove(); // Remove from DOM; the animation loop handles re-insertion
                    }

                    // 4. Reset component state variables
                    is_now_running.set(false);

                    let mut finished_signal = props.finished;
                    if spans_reference.borrow().is_empty() {
                        finished_signal.set(true); // Nothing to type, inherently finished
                    } else {
                        finished_signal.set(false); // Text awaits typing
                    }

                    // 5. Consume the reset signal to prevent looping
                    let mut reset_signal = props.do_reset;
                    reset_signal.set(false);
                }
            });

            // ------------------------------------------------------------------------
            // Effect 3: The Animation Engine
            // ------------------------------------------------------------------------
            #[cfg(feature = "web")]
            use_effect(move || {
                let spans_reference = text_node_list.read().clone();

                // Start conditions: component mounted, visible on screen, initiated, not currently active, and has text to type.
                if let Some(_) = mounted_target()
                    && (props.initiate)()
                    && !is_now_running()
                    && !spans_reference.borrow().is_empty()
                {
                    is_now_running.set(true);
                    props.initiate.set(false); // Consume the initiate signal

                    let active_cursor = cursor_element.read().clone();
                    let weak_loop_tracker = Rc::downgrade(&animation_loop);

                    let mut finished_signal = props.finished;
                    let delay_ms_signal = props.delay_ms;
                    let on_event_callback = props.on_event.clone();

                    let performance = web_sys::window().unwrap().performance().unwrap();

                    // Initialize the math/tracking state for the typing progress
                    let progress_state = Rc::new(RefCell::new(AnimationProgressState {
                        start_time: -1.0,
                        accumulated_cost_in_span: 0.0,
                        total_cost_processed: 0.0,
                        span_index: 0,
                        char_index: 0,
                        current_cursor_span_index: None,
                        last_cursor_class: "",
                        last_cursor_text: "".to_string(),
                    }));

                    // Create the closure that will run every monitor frame
                    let animation_closure = Closure::new(move || {
                        // Safely attempt to get our loop tracker
                        let tracker_rc = match weak_loop_tracker.upgrade() { Some(t) => t, _ => return };
                        let mut tracker_mut = tracker_rc.borrow_mut();

                        let current_time = performance.now();
                        let mut state = progress_state.borrow_mut();
                        let script_spans = spans_reference.borrow();

                        // Helper function to conclude the animation cleanly
                        let mut finalize_animation = || {
                            // Force all text to display instantly in case of skipped frames
                            for span in script_spans.iter() {
                                let full_text: String = span.data.iter().map(|d| d.char).collect();
                                span.node.set_data(&full_text);
                            }
                            // Clean up the cursor
                            if let Some(cursor) = active_cursor.as_ref() {
                                cursor.remove();
                            }
                            // Fire final trigger if one exists on the very last node
                            if let Some(last_span) = script_spans.last() {
                                if let Some(trigger_name) = &last_span.trigger {
                                    if let Some(callback) = &on_event_callback { callback.call(trigger_name.clone()); }
                                }
                            }
                            finished_signal.set(true);
                            tracker_mut.request_id = None;
                        };

                        // Set the starting timestamp on the first frame
                        if state.start_time < 0.0 { state.start_time = current_time; }

                        // Get the delay using .peek() to avoid subscribing inside the loop
                        let delay = *delay_ms_signal.peek();
                        if delay <= 0.0 {
                            return finalize_animation();
                        }

                        let time_budget = (current_time - state.start_time) / delay;

                        // A buffer to batch characters so we only cross the WASM boundary once per span
                        let mut char_batch = String::new();

                        // Ensure the cursor DOM element is inserted next to the currently typing span
                        if state.span_index < script_spans.len() && state.current_cursor_span_index != Some(state.span_index) {
                            if let Some(cursor) = active_cursor.as_ref() {
                                let current_node = &script_spans[state.span_index].node;
                                if let Some(parent) = current_node.parent_node() {
                                    let _ = parent.insert_before(cursor, current_node.next_sibling().as_ref());
                                    state.current_cursor_span_index = Some(state.span_index);
                                }
                            }
                        }

                        // Main parsing loop
                        while state.span_index < script_spans.len() {
                            let current_span = &script_spans[state.span_index];

                            if current_span.data.is_empty() {
                                state.span_index += 1;
                                state.current_cursor_span_index = None;
                                continue;
                            }

                            let current_char_data = &current_span.data[state.char_index];

                            // Check if we have enough budget
                            if time_budget >= (state.total_cost_processed + state.accumulated_cost_in_span + current_char_data.cost) {

                                // Batch the character instead of appending to the DOM immediately
                                char_batch.push(current_char_data.char);

                                // Update our costs and indices
                                state.accumulated_cost_in_span += current_char_data.cost;
                                state.char_index += 1;

                                // If we finished this span, move to the next one
                                if state.char_index >= current_span.data.len() {
                                    // Flush the batch to the DOM before moving to the next span
                                    if !char_batch.is_empty() {
                                        let _ = current_span.node.append_data(&char_batch);
                                        char_batch.clear();
                                    }

                                    state.total_cost_processed += state.accumulated_cost_in_span;
                                    state.span_index += 1;
                                    state.char_index = 0;
                                    state.accumulated_cost_in_span = 0.0;
                                    state.current_cursor_span_index = None;

                                    // Trigger events...
                                    let current_trigger = &current_span.trigger;
                                    let next_trigger = if state.span_index < script_spans.len() { &script_spans[state.span_index].trigger } else { &None };

                                    if let Some(trigger_id) = current_trigger {
                                        if current_trigger != next_trigger {
                                            if let Some(callback) = &on_event_callback { callback.call(trigger_id.clone()); }
                                        }
                                    }
                                }
                            } else {
                                // Flush any pending batch before ending the frame
                                if !char_batch.is_empty() {
                                    let _ = current_span.node.append_data(&char_batch);
                                }

                                // We are OUT of budget. Handle cursor visualization with CACHING.
                                if let Some(cursor) = active_cursor.as_ref() {
                                    if current_char_data.is_cjk {
                                        let target_class = "typing-cursor";
                                        if state.last_cursor_class != target_class {
                                            cursor.set_class_name(target_class);
                                            state.last_cursor_class = target_class;
                                        }

                                        let progress_ratio = time_budget - (state.total_cost_processed + state.accumulated_cost_in_span);
                                        let frame_index = ((progress_ratio / current_char_data.cost) * 6.0) as u32 + 1;
                                        let visual_char = char::from_u32(0xFF00 + frame_index.clamp(1, 6)).unwrap().to_string();

                                        if state.last_cursor_text != visual_char {
                                            cursor.set_text_content(Some(&visual_char));
                                            state.last_cursor_text = visual_char;
                                        }
                                    } else {
                                        let target_class = "typing-cursor blinking";
                                        if state.last_cursor_class != target_class {
                                            cursor.set_class_name(target_class);
                                            state.last_cursor_class = target_class;

                                            // Only clear text content if we actually swapped classes
                                            cursor.set_text_content(None);
                                            state.last_cursor_text.clear();
                                        }
                                    }
                                }
                                break;
                            }
                        }

                        // Schedule next frame or finalize if done
                        if state.span_index >= script_spans.len() {
                            finalize_animation();
                        } else {
                            let window = web_sys::window().unwrap();
                            tracker_mut.request_id = Some(window.request_animation_frame(tracker_mut._callback.as_ref().unwrap().as_ref().unchecked_ref()).unwrap());
                        }
                    });

                    // Kick off the very first animation frame
                    let window = web_sys::window().unwrap();
                    let frame_id = window.request_animation_frame(animation_closure.as_ref().unchecked_ref()).unwrap();

                    let mut tracker_lock = animation_loop.borrow_mut();
                    tracker_lock.request_id = Some(frame_id);
                    tracker_lock._callback = Some(animation_closure);
                }
            });

            rsx! {
                $tag {
                    "data-typed-container": "true",
                    role: "text",
                    aria_label: "{extracted_text}",
                    onmounted: move |event| mounted_target.set(Some(event.data())),
                    onvisible: move |event| is_now_visible.set(event.data().is_intersecting().unwrap_or(false)),
                    ..props.attributes,
                    { props.children }
                }
            }
        }
    };
}


generate_typed_component!(TypedDiv, div);
generate_typed_component!(TypedSpan, span);
generate_typed_component!(TypedP, p);