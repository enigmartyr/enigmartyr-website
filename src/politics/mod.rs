use crate::generic::web::{snapping_scroll_div::*, typed_div::*};
use crate::politics::component_manilla_folder::*;
use crate::Route;
use dioxus::prelude::*;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use gloo_net::http::Request;

#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use wasm_bindgen::{prelude::*, JsCast};

pub mod component_manilla_folder;

// ==========================================
// 1. CORE UTILITIES & API
// ==========================================

static RE_CJK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\u{4e00}-\u{9fff}\u{3000}-\u{303f}\u{ff00}-\u{ffef}]").unwrap());

async fn fetch_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, String> {
    let sep = if url.contains('?') { "&" } else { "?" };
    let cache_busted_url = format!("{url}{sep}v={}", js_sys::Date::now());

    let res = Request::get(&cache_busted_url)
        .header("Cache-Control", "no-cache, no-store, must-revalidate")
        .header("Pragma", "no-cache")
        .header("Expires", "0")
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !res.ok() {
        return Err(format!("HTTP {}: {url}", res.status()));
    }

    res.json::<T>()
        .await
        .map_err(|e| format!("Serde Error: {e}"))
}

fn scroll_to_source_file(file_index: usize) {
    #[cfg(feature = "web")]
    if let Some(element) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(&format!("source-file-{file_index}")))
    {
        let mut options = web_sys::ScrollIntoViewOptions::new();
        options.behavior(web_sys::ScrollBehavior::Smooth);
        options.block(web_sys::ScrollLogicalPosition::Nearest);
        options.inline(web_sys::ScrollLogicalPosition::Nearest);
        let _ = element.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

// ==========================================
// 2. DATA MODELS
// ==========================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File { pub internal_source: String, pub external_source: String, pub source_descript: String, pub source_citation: String }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Portraiture { pub normal: String, pub sketch: String, pub source: String }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name { pub full: String, pub last: String }

#[derive(Debug, Clone, PartialEq)]
pub enum TermYear {
    Unknown,   // null  → unknown start/end
    DeadEnd,   // -1    → term ends at person's death
    ValueYr(u32), // positive year
}

impl Serialize for TermYear {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TermYear::Unknown => serializer.serialize_none(),
            TermYear::DeadEnd => serializer.serialize_i32(-1),
            TermYear::ValueYr(year) => serializer.serialize_u32(*year),
        }
    }
}

impl<'de> Deserialize<'de> for TermYear {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Re-use the exact same deserialization that Option<i32> used before,
        // then convert the three cases we care about.
        match Option::<i32>::deserialize(deserializer)? {
            None => Ok(TermYear::Unknown),
            Some(-1) => Ok(TermYear::DeadEnd),
            Some(n) if n >= 0 => Ok(TermYear::ValueYr(n as u32)),
            Some(n) => Err(serde::de::Error::custom(format!("invalid term year: {}", n))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Appointment {
    pub term: (TermYear, TermYear),
    pub name: String,
    pub subs: Vec<Appointment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: Name,
    pub span: (i32, i32),
    pub portraiture: Option<Portraiture>,
    #[serde(default)]
    pub backgrounds: Vec<String>,
    #[serde(default)]
    pub appointment: Vec<Appointment>,
}

#[derive(Clone, PartialEq)]
pub struct SubBackground {
    pub icon_path: String,
    pub pseudonym: String,
    pub is_leader: bool
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageBlock { pub id: String, pub title: String, pub uri: String }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageChapter { pub id: String, pub title: String, pub uri: String, pub blocks: Vec<PageBlock> }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageSection { pub id: String, pub title: String, pub uri: String, pub chapters: Vec<PageChapter> }

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParagraphType { Initial, Typical, Excerpt, NestedExcerpt }

impl ParagraphType {
    pub const fn as_class(&self) -> &'static str {
        match self { Self::Initial => "p-type-initial", Self::Typical => "p-type-typical", Self::Excerpt => "p-type-excerpt", Self::NestedExcerpt => "p-type-nested-excerpt" }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paragraph { pub p_type: ParagraphType, pub p_text: Vec<TextSegment> }

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PureTextLang { ZH, EN, LA }

impl PureTextLang {
    pub const fn as_str(&self) -> &'static str { match self { Self::ZH => "zh", Self::EN => "en", Self::LA => "la" } }
    pub const fn flex_ratios(&self) -> (&'static str, &'static str) { match self { Self::ZH => ("3", "1"), _ => ("1", "1") } }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum TextSegment {
    PureText { lang: PureTextLang, content: String },
    FigRef(String), FileRef(String), FileIdx(usize),
    Marked(Vec<TextSegment>), Inserted(Vec<TextSegment>), Deleted(Vec<TextSegment>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum BlockContent {
    Subtitle(String),
    Paragraph(Paragraph),
    Translated { lang: PureTextLang, left: Paragraph, right: Paragraph },
}

#[derive(Clone, PartialEq)]
pub enum RenderGroup {
    Subtitle(String),
    Paragraph(usize, Paragraph),
    Translated(PureTextLang, Vec<(usize, Paragraph, Paragraph)>),
}

#[derive(Clone, PartialEq)]
pub struct CompiledDocument {
    pub groups: Vec<RenderGroup>,
    pub manifest: Vec<File>,
    pub figures: Vec<(String, usize, String)>, // (person_id, paragraph_seq, trigger_id)
}

// ==========================================
// 3. PURE DATA TRANSFORMATIONS
// ==========================================

fn resolve_context(path: &[String], sections: &[PageSection]) -> (Vec<String>, String) {
    let mut ids = Vec::new();
    let mut title = String::new();

    let Some(s_uri) = path.first() else { return (ids, title) };
    let Some((s_idx, sec)) = sections.iter().enumerate().find(|(_, s)| &s.uri == s_uri) else { return (ids, title) };
    ids.push(sec.id.clone());
    let s_char = (b'A' + s_idx as u8) as char;
    title = format!("{s_char}: {}", sec.title);

    let Some(c_uri) = path.get(1) else { return (ids, title) };
    let Some((c_idx, chap)) = sec.chapters.iter().enumerate().find(|(_, c)| &c.uri == c_uri) else { return (ids, title) };
    ids.push(chap.id.clone());
    title = format!("{s_char} {}.0: {}", c_idx + 1, chap.title);

    let Some(b_uri) = path.get(2) else { return (ids, title) };
    let Some((b_idx, block)) = chap.blocks.iter().enumerate().find(|(_, b)| &b.uri == b_uri) else { return (ids, title) };
    ids.push(block.id.clone());
    title = format!("{s_char} {}.{}: {}", c_idx + 1, b_idx + 1, block.title);

    (ids, title)
}

fn resolve_background(bg_str: &str) -> Vec<SubBackground> {
    // TODO: Map string values like "Veteran(Army)" to your specific SVG paths.
    vec![SubBackground {
        icon_path: "M0 0h24v24H0z".to_string(), // Replace with actual path
        pseudonym: bg_str.to_string(),
        is_leader: false,
    }]
}

fn map_pointers(
    segs: &mut [TextSegment],
    reg: &HashMap<String, File>,
    man: &mut Vec<File>,
    ptrs: &mut HashMap<String, usize>,
    figures: &mut Vec<(String, usize, String)>,
    seen_figs: &mut HashSet<String>,
    p_id: usize,
    path: String
) {
    for (i, seg) in segs.iter_mut().enumerate() {
        let current_path = if path.is_empty() { format!("{i}") } else { format!("{path}-{i}") };

        match seg {
            TextSegment::FileRef(id) => {
                if let Some(&idx) = ptrs.get(id) {
                    *seg = TextSegment::FileIdx(idx);
                } else if let Some(file) = reg.get(id) {
                    let idx = man.len();
                    man.push(file.clone());
                    ptrs.insert(id.clone(), idx);
                    *seg = TextSegment::FileIdx(idx);
                }
            }
            TextSegment::FigRef(id) => {
                if seen_figs.insert(id.clone()) {
                    figures.push((id.clone(), p_id, format!("fig-{p_id}-{current_path}")));
                }
            }
            TextSegment::Marked(c) | TextSegment::Inserted(c) | TextSegment::Deleted(c) => {
                map_pointers(c, reg, man, ptrs, figures, seen_figs, p_id, current_path);
            }
            _ => {}
        }
    }
}

fn compile_document(mut blocks: Vec<BlockContent>, registry: &HashMap<String, File>) -> CompiledDocument {
    let mut manifest = Vec::new();
    let mut pointers = HashMap::new();
    let mut groups = Vec::new();
    let mut figures = Vec::new();
    let mut seen_figs = HashSet::new();
    let mut trans_group = Vec::new();
    let mut cur_lang = Option::<PureTextLang>::None;
    let mut seq = 1usize;

    for content in blocks.iter_mut() {
        match content {
            BlockContent::Translated { lang, left, right } => {
                map_pointers(&mut left.p_text, registry, &mut manifest, &mut pointers, &mut figures, &mut seen_figs, seq, "L".to_string());
                map_pointers(&mut right.p_text, registry, &mut manifest, &mut pointers, &mut figures, &mut seen_figs, seq, "R".to_string());

                if Some(*lang) != cur_lang && !trans_group.is_empty() {
                    groups.push(RenderGroup::Translated(cur_lang.unwrap(), std::mem::take(&mut trans_group)));
                }
                cur_lang = Some(*lang);
                trans_group.push((seq, left.clone(), right.clone()));
                seq += 1;
            }
            other => {
                // FIX 1: Flush the trans_group immediately when hitting any non-translated block
                if !trans_group.is_empty() {
                    groups.push(RenderGroup::Translated(cur_lang.unwrap(), std::mem::take(&mut trans_group)));
                    cur_lang = None;
                }
                match other {
                    BlockContent::Subtitle(text) => groups.push(RenderGroup::Subtitle(text.clone())),
                    BlockContent::Paragraph(p) => {
                        map_pointers(&mut p.p_text, registry, &mut manifest, &mut pointers, &mut figures, &mut seen_figs, seq, String::new());
                        groups.push(RenderGroup::Paragraph(seq, p.clone()));
                        seq += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    if !trans_group.is_empty() {
        groups.push(RenderGroup::Translated(cur_lang.unwrap(), trans_group));
    }

    CompiledDocument { groups, manifest, figures }
}

// ==========================================
// 4. CUSTOM HOOKS
// ==========================================

#[derive(Clone, Copy, PartialEq)]
struct TypingController {
    highest: Signal<usize>,
    completions: Signal<HashMap<usize, HashSet<String>>>,
    figures: Signal<HashSet<String>>,
}

impl TypingController {
    fn new() -> Self {
        Self {
            highest: use_signal(|| 0usize),
            completions: use_signal(HashMap::new),
            figures: use_signal(HashSet::new),
        }
    }

    fn register_hit(&mut self, trigger_id: String, seq: usize, required: usize) {
        if trigger_id.starts_with("fig-") {
            self.figures.write().insert(trigger_id);
            return;
        }

        let mut map = self.completions.write();
        let set = map.entry(seq).or_default();
        set.insert(trigger_id);
        if set.len() >= required && seq >= (self.highest)() {
            self.highest.set(seq + 1);
        }
    }
}

// ==========================================
// 5. TOP LEVEL ROUTING
// ==========================================

#[component]
pub fn Politics() -> Element {
    rsx! { div { h1 { "Politics Hub" } p { "Select an archive." } } }
}

#[component]
pub fn PoliticsPage(region: String, topic: String, content_path: Vec<String>) -> Element {
    let nav = use_navigator();
    let topic_path = format!("{region}/{topic}");
    let clean_path: Vec<String> = content_path.into_iter().filter(|s| !s.is_empty()).collect();
    let active_uri = clean_path.join("/");

    let index_res = use_resource(use_reactive!(|topic_path| async move { fetch_json::<Vec<PageSection>>(&format!("/data/politics/{topic_path}/index.json")).await }));

    use_effect(use_reactive!(|active_uri, index_res, region, topic| {
        if active_uri.is_empty() {
            if let Some(Ok(sections)) = &*index_res.read() {
                if let Some(first) = sections.first() {
                    nav.replace(Route::PoliticsPage {
                        region: region.clone(),
                        topic: topic.clone(),
                        content_path: vec![first.uri.clone()]
                    });
                }
            }
        }
    }));

    match &*index_res.read() {
        None => rsx! { div { class: "loading-text", "Loading..." } },
        Some(Err(e)) => rsx! { div { style: "color: red", "Error: {e}" } },
        Some(Ok(_)) if active_uri.is_empty() => rsx! { div { "Redirecting..." } },
        Some(Ok(sections)) => rsx! {
            ArchiveWorkspace {
                region,
                topic,
                topic_path,
                clean_path,
                active_uri,
                sections: sections.clone()
            }
        }
    }
}

// ==========================================
// 6. WORKSPACE COMPONENTS
// ==========================================

#[component]
fn ArchiveWorkspace(region: String, topic: String, topic_path: String, clean_path: Vec<String>, active_uri: String, sections: Vec<PageSection>) -> Element {
    let (id_path, display_title) = resolve_context(&clean_path, &sections);
    let registry_res = use_resource(use_reactive!(|topic_path| async move { fetch_json::<HashMap<String, File>>(&format!("/data/politics/{topic_path}/docs/index.json")).await }));

    let Some(Ok(registry)) = &*registry_res.read() else {
        return rsx! { div { "Loading database..." } };
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/style/politics.css") }
        div { id: "page-container",
            SidebarNav { sections, active_uri: active_uri.clone(), region, topic }

            div { id: "chapter-container",
                for uri in std::iter::once(active_uri.clone()) {
                    ChapterView {
                        key: "{uri}",
                        topic_path: topic_path.clone(),
                        id_path: id_path.clone(),
                        display_title: display_title.clone(),
                        registry: registry.clone()
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarNav(sections: Vec<PageSection>, active_uri: String, region: String, topic: String) -> Element {
    rsx! {
        TypedDiv { id: "nav-container",
            SnappingScrollDiv { id: "nav-grid",
                for (s_idx, s) in sections.into_iter().enumerate() {
                    NavEntry { title: s.title, id_str: format!("   {}   ", (b'A' + s_idx as u8) as char), class: "nav-entry-section", active: active_uri == s.uri, to: vec![s.uri.clone()], region: region.clone(), topic: topic.clone() }
                    for (c_idx, c) in s.chapters.into_iter().enumerate() {
                        NavEntry { title: c.title, id_str: format!(" {:>2}.0  ", c_idx + 1), class: "nav-entry-chapter", active: active_uri == format!("{}/{}", s.uri, c.uri), to: vec![s.uri.clone(), c.uri.clone()], region: region.clone(), topic: topic.clone() }
                        for (b_idx, b) in c.blocks.into_iter().enumerate() {
                            NavEntry { title: b.title, id_str: format!(" {:>2}.{:<2} ", c_idx + 1, b_idx + 1), class: "nav-entry-block", active: active_uri == format!("{}/{}/{}", s.uri, c.uri, b.uri), to: vec![s.uri.clone(), c.uri.clone(), b.uri.clone()], region: region.clone(), topic: topic.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavEntry(title: String, id_str: String, class: &'static str, active: bool, to: Vec<String>, region: String, topic: String) -> Element {
    rsx! {
        Link {
            class: format!("nav-entry {class} {}", if active { "active" } else { "" }),
            to: Route::PoliticsPage { region, topic, content_path: to },
            span { class: "nav-entry-id", span { class: "nav-word", for ch in id_str.chars() { span { class: "nav-char", "{ch}" } } } }
            span { class: "nav-entry-title", for word in title.split_whitespace() { span { class: "nav-word", for ch in (word.to_owned() + " ").chars() { span { class: "nav-char", "{ch}" } } } } }
        }
    }
}

#[component]
fn ChapterView(topic_path: String, id_path: Vec<String>, display_title: String, registry: HashMap<String, File>) -> Element {
    let mut is_panel_focused = use_signal(|| false);
    let hovered_fig = use_signal(|| Option::<String>::None);
    let controller = TypingController::new();

    let block_res = use_resource(use_reactive!(|topic_path, id_path| async move {
        let url = match id_path.as_slice() {
            [s] => format!("/data/politics/{topic_path}/{s}/s.json"),
            [s, c] => format!("/data/politics/{topic_path}/{s}/{c}/c.json"),
            [s, c, b] => format!("/data/politics/{topic_path}/{s}/{c}/{b}.json"),
            _ => return Err("Invalid path depth".to_string()),
        };
        fetch_json::<Vec<BlockContent>>(&url).await
    }));

    let doc = use_memo(move || compile_document(block_res.read().as_ref().and_then(|r| r.as_ref().ok()).cloned().unwrap_or_default(), &registry));

    let figure_ids = use_memo(move || {
        let mut ids = doc.read().figures.iter().map(|f| f.0.clone()).collect::<Vec<_>>();
        ids.sort(); ids.dedup();
        ids
    });

    let person_registry_res = use_resource(use_reactive!(|figure_ids| async move {
        let mut reg = HashMap::new();
        let ids = figure_ids.read().clone();
        for id in ids {
            if let Ok(person) = fetch_json::<Person>(&format!("/data/politics/person/{id}.json")).await {
                reg.insert(id, person);
            }
        }
        reg
    }));

    if block_res.read().is_none() { return rsx! { div { class: "loading-text", "Loading content..." } }; }
    if let Some(Err(e)) = &*block_res.read() { return rsx! { div { "Error: {e}" } }; }

    // STRICT BLOCK: Do not mount ArticleContent until every required figure is in the registry.
    // This physically prevents TypedP from permanently locking in a missing text string.
    let person_reg = person_registry_res.read().clone().unwrap_or_default();
    let missing_figures = doc.read().figures.iter().any(|f| !person_reg.contains_key(&f.0));

    if missing_figures {
        return rsx! { div { class: "loading-text", "Loading figures..." } };
    }

    rsx! {
        div { class: "chapter-block body-block", onvisible: move |_| is_panel_focused.set(false),
            div { class: "body-container",
                ArticleContent {
                    title: display_title,
                    doc: doc.read().clone(),
                    controller,
                    is_panel_focused,
                    hovered_fig,
                    person_reg
                }
                SourcePanel { files: doc.read().manifest.clone(), is_panel_focused }
            }
        }
    }
}



#[component]
fn ParagraphLine(
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    p: Paragraph,
    seq: usize,
    id_str: String,
    req: usize,
    mut controller: TypingController,
    focus: Signal<bool>,
    hov: Signal<Option<String>>,
    person_reg: HashMap<String, Person>
) -> Element {
    let mut initiate = use_signal(|| false);

    // 1. The definitive hook for when this specific paragraph finishes
    let mut is_finished = use_signal(|| false);

    // 2. Start typing only when it is our turn
    use_effect(move || {
        let ready = (controller.highest)() >= seq;
        if ready && !initiate() && !is_finished() {
            initiate.set(true);
        }
    });

    // 3. Advance the sequence ONLY when the Typed component explicitly declares it is fully done
    use_effect(move || {
        if is_finished() {
            let mut map = controller.completions.write();
            let set = map.entry(seq).or_default();

            // Register this specific paragraph as complete
            if !set.contains(&id_str) {
                set.insert(id_str.clone());

                // If all required paragraphs for this seq index are done, trigger the next one
                if set.len() >= req && seq >= (controller.highest)() {
                    controller.highest.set(seq + 1);
                }
            }
        }
    });

    rsx! {
        TypedP {
            initiate: initiate,
            finished: is_finished, // Bound directly to your TypedDivProps
            class: p.p_type.as_class(),
            attributes,

            // on_event is now ONLY for tracking figures.
            // It has zero influence on when the next paragraph starts.
            on_event: move |trigger: String| {
                if trigger.starts_with("fig-") {
                    controller.figures.write().insert(trigger);
                }
            },

            for (s_idx, seg) in p.p_text.into_iter().enumerate() {
                 SegmentNode {
                    key: "seg-{seq}-{s_idx}",
                    seg,
                    focus,
                    path: format!("{s_idx}"),
                    hov,
                    person_reg: person_reg.clone(),
                    p_id: seq
                }
            }
        }
    }
}

#[component]
fn SegmentNode(
    seg: TextSegment,
    mut focus: Signal<bool>,
    path: String,
    mut hov: Signal<Option<String>>,
    person_reg: HashMap<String, Person>,
    p_id: usize
) -> Element {
    match seg {
        TextSegment::PureText { lang, content } => rsx!(span { lang: lang.as_str(), dangerous_inner_html: content }),
        TextSegment::FigRef(id) => {
            // Retrieve full name from person_reg using the ID from FigRef
            let name = person_reg.get(&id)
                .map(|p| p.name.full.clone())
                .unwrap_or_else(|| id.clone()); // Fallback to ID only if registry fails

            let trigger_id = format!("fig-{p_id}-{path}");
            let is_hovered = hov() == Some(name.clone());
            let class_str = if is_hovered { "figure-citation-container figure-hovered" } else { "figure-citation-container" };

            let pn_enter = name.clone();
            let pn_leave = name.clone();

            rsx! {
                span {
                    class: "{class_str}",
                    "data-trigger": "{trigger_id}",
                    onmouseenter: move |_| hov.set(Some(pn_enter.clone())),
                    onmouseleave: move |_| if hov() == Some(pn_leave.clone()) { hov.set(None); },
                    "{name}"
                }
            }
        },
        TextSegment::FileRef(_) => rsx! { "ERR_UNCOMPILED_FILE" },
        TextSegment::FileIdx(idx) => rsx! {
            span { class: "source-citation-container",
                span { class: "source-citation glitch", "data-text": idx + 1, onclick: move |_| { focus.set(true); scroll_to_source_file(idx); }, "{ idx + 1 }" }
            }
        },
        TextSegment::Marked(c) => rsx! { mark { for (i, child) in c.into_iter().enumerate() { SegmentNode { seg: child, focus, path: format!("{path}-{i}"), hov, person_reg: person_reg.clone(), p_id } } } },
        TextSegment::Inserted(c) => rsx! { ins { for (i, child) in c.into_iter().enumerate() { SegmentNode { seg: child, focus, path: format!("{path}-{i}"), hov, person_reg: person_reg.clone(), p_id } } } },
        TextSegment::Deleted(c) => rsx! { del { for (i, child) in c.into_iter().enumerate() { SegmentNode { seg: child, focus, path: format!("{path}-{i}"), hov, person_reg: person_reg.clone(), p_id } } } },
    }
}


// ==========================================
// 7. ARTICLE & PARAGRAPH RENDERERS
// ==========================================

#[component]
fn ArticleContent(
    title: String,
    doc: CompiledDocument,
    mut controller: TypingController,
    is_panel_focused: Signal<bool>,
    hovered_fig: Signal<Option<String>>,
    person_reg: HashMap<String, Person>
) -> Element {
    let initiate_title = use_signal(|| true);

    rsx! {
        div { class: "body-segment text-segment",
            TypedDiv { style: "display: contents;", initiate: initiate_title, on_event: move |id: String| { if id == "title" { controller.highest.set(1); } },
                div { class: "text-title", "data-trigger": "title", "{title}" }
            }
            div { class: "text-contents",
                SnappingScrollDiv {
                    div {
                        style: "position: relative; z-index: 1;",
                        class: "float-container",
                        for (person_id, p_id, trigger_id) in doc.figures.iter() {
                            if let Some(person) = person_reg.get(person_id) {
                                FigureCard {
                                    key: "{trigger_id}",
                                    person: person.clone(),
                                    trigger_paragraph_id: *p_id,
                                    trigger_id: trigger_id.clone(),
                                    controller,
                                    hov: hovered_fig
                                }
                            }
                        }
                    }

                    for (g_idx, group) in doc.groups.into_iter().enumerate() {
                        match group {
                            RenderGroup::Subtitle(text) => rsx! { div { class: "block-subtitle", key: "sub-{g_idx}", "{text}" } },
                            RenderGroup::Paragraph(seq, p) => rsx! {
                                ParagraphLine { key: "p-{seq}", p, seq, id_str: format!("{seq}"), req: 1, controller, focus: is_panel_focused, hov: hovered_fig, person_reg: person_reg.clone() }
                            },
                            RenderGroup::Translated(lang, duals) => rsx! {
                                div { class: "dual-paragraph-container", key: "trans-{g_idx}", style: format!("display: grid; grid-template-columns: {}fr {}fr;", lang.flex_ratios().0, lang.flex_ratios().1),
                                    for (r_idx, (seq, left, _)) in duals.iter().enumerate() {
                                        ParagraphLine { key: "pl-{seq}", style: format!("grid-column: 1; grid-row: {};", r_idx + 1), p: left.clone(), seq: *seq, id_str: format!("{seq}-L"), req: 2, controller, focus: is_panel_focused, hov: hovered_fig, person_reg: person_reg.clone() }
                                    }
                                    for (r_idx, (seq, _, right)) in duals.iter().enumerate() {
                                        ParagraphLine { key: "pr-{seq}", style: format!("grid-column: 2; grid-row: {};", r_idx + 1), p: right.clone(), seq: *seq, id_str: format!("{seq}-R"), req: 2, controller, focus: is_panel_focused, hov: hovered_fig, person_reg: person_reg.clone() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==========================================
// 8. RESTORED FIGURE CARD COMPONENT
// ==========================================

const FIGURE_BACKGROUND_VERTICAL_ALLOWANCE: usize = 6;

#[component]
fn FigureCard(
    person: Person,
    trigger_paragraph_id: usize,
    trigger_id: String,
    controller: TypingController,
    mut hov: Signal<Option<String>>,
) -> Element {
    let mut east_side_bg = Vec::new();
    let mut west_side_bg = Vec::new();
    let mut west_side_count = 0;

    for bg_str in &person.backgrounds {
        let sub_bgs = resolve_background(bg_str);
        let sub_len = sub_bgs.len();
        if west_side_count + sub_len <= FIGURE_BACKGROUND_VERTICAL_ALLOWANCE {
            west_side_bg.push(sub_bgs);
            west_side_count += sub_len;
        } else {
            east_side_bg.push(sub_bgs);
        }
    }

    let is_revealed = use_memo(move || {
        (controller.highest)() > trigger_paragraph_id || controller.figures.read().contains(&trigger_id)
    });

    let mut image_element = use_signal(|| Option::<std::rc::Rc<MountedData>>::None);
    let mut typing_initiate = use_signal(|| false);
    let typing_delay_ms = use_signal(|| 10.0);
    let mut typing_do_reset = use_signal(|| false);

    #[cfg(feature = "web")]
    use_effect(move || {
        if let Some(event_data) = image_element().map(|e| e.as_web_event()) {
            if let Ok(img_node) = event_data.dyn_into::<web_sys::HtmlImageElement>() {
                let src = img_node.src();
                img_node.set_src("");
                img_node.set_src(&src);
            }
        }
    });

    let person_name = person.name.full.clone();
    let is_hovered = hov() == Some(person_name.clone());

    let base_class = if is_revealed() { "figure-container figure-revealed" } else { "figure-container" };
    let hover_class = if is_hovered { "figure-hovered" } else { "" };

    let pn_enter = person_name.clone();
    let pn_leave = person_name.clone();

    rsx! {
        div {
            class: "{base_class} {hover_class}",
            onmouseenter: move |_| {
                hov.set(Some(pn_enter.clone()));
                typing_initiate.set(true);
            },
            onmouseleave: move |_| {
                if hov() == Some(pn_leave.clone()) { hov.set(None); }
                typing_do_reset.set(true);
            },
            div {
                class: "figure-portrait-container",
                if let Some(portrait) = &person.portraiture {
                    img {
                        onmounted: move |event| image_element.set(Some(event.data())),
                        class: "figure-portrait figure-portrait-sketch",
                        src: portrait.sketch.trim()
                    }
                    div {
                        class: "figure-portrait figure-portrait-scan",
                        style: "mask-image: url({portrait.normal.trim()});"
                    }
                    img {
                        class: "figure-portrait figure-portrait-normal",
                        src: portrait.normal.trim()
                    }
                }
                TypedDiv {
                    class: "figure-background-container",
                    initiate: typing_initiate,
                    delay_ms: typing_delay_ms,
                    do_reset: typing_do_reset,
                    div {
                        class: "figure-background-container-west",
                        for background in west_side_bg {
                            div {
                                class: "figure-background",
                                for sub_bg in background {
                                    div {
                                        class: "figure-sub-background",
                                        svg {
                                            view_box: "0 0 32 32",
                                            fill: "currentColor",
                                            style: if sub_bg.is_leader { "color: goldenrod;" } else { "" },
                                            path { d: sub_bg.icon_path }
                                        }
                                        span { class: "figure-sub-background-name", " { sub_bg.pseudonym }" }
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "figure-background-container-east",
                        for background in east_side_bg {
                            div {
                                class: "figure-background",
                                for sub_bg in background {
                                    div {
                                        class: "figure-sub-background",
                                        svg {
                                            view_box: "0 0 32 32",
                                            fill: "currentColor",
                                            style: if sub_bg.is_leader { "color: goldenrod;" } else { "" },
                                            path { d: sub_bg.icon_path }
                                        }
                                        span { class: "figure-sub-background-name", "{ sub_bg.pseudonym } " }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "figure-name-container",
                div {
                    class: "figure-name-background",
                    span {
                        class: "figure-name",
                        "data-name-len": person.name.full.len(),
                        "data-name-str": person.name.full.clone()
                    }
                }
            }
        }
    }
}

// ==========================================
// 9. SOURCE VIEWER
// ==========================================

#[component]
fn SourcePanel(files: Vec<File>, mut is_panel_focused: Signal<bool>) -> Element {
    let mut current_idx = use_signal(|| 0usize);
    let total = files.len();
    let idx = current_idx();

    rsx! {
        div { class: if is_panel_focused() { "body-segment source-segment source-segment-active" } else { "body-segment source-segment" },
            div { class: "source-counter-area",
                div { class: "source-deactivate-button-area",
                    div { class: if !is_panel_focused() { "bipolar-button disabled" } else { "bipolar-button" },
                        style: "--button-image-still: url({asset!(\"/assets/icon/view-locked.svg\")}); --button-image-hover: url({asset!(\"/assets/icon/view-unlocked.svg\")});",
                        onclick: move |_| is_panel_focused.set(false)
                    }
                },
                div { class: "source-counter",
                    span { class: if idx == 0 { "source-button disabled" } else { "source-button" }, onclick: move |_| scroll_to_source_file(idx.saturating_sub(1)), "<" }
                    span { "{ (idx + 1).min(total) } of { total }" }
                    span { class: if idx + 1 >= total { "source-button disabled" } else { "source-button" }, onclick: move |_| scroll_to_source_file(idx + 1), ">" }
                }
            }
            div { class: "source-container",
                for (file_idx, file) in files.into_iter().enumerate() {
                    div { class: "source-area", id: "source-file-{file_idx}",
                        onvisible: move |e| if let Ok(true) = e.data().is_intersecting() { current_idx.set(file_idx); },
                        ManillaFolder { file_ref: file }
                    }
                }
            }
        }
    }
}