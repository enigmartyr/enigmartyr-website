use dioxus::prelude::*;

use std::collections::HashSet;

use std::sync::atomic::{AtomicUsize, Ordering};

use std::hash::{DefaultHasher, Hash, Hasher};

use wasm_bindgen::prelude::*;

use js_sys::{Promise, Reflect};

use wasm_bindgen_futures::JsFuture;

use web_sys::{HtmlCanvasElement, HtmlElement};

use crate::politics::File;

pub const MANILLA_FOLDER_FACE: Asset = asset!("/assets/icon/manilla-folder-face.svg");

pub const MANILLA_FOLDER_BACK: Asset = asset!("/assets/icon/manilla-folder-back.svg");

const FILE_LOCAL_STILL: Asset = asset!("/assets/icon/file-local-still.svg");

const FILE_LOCAL_HOVER: Asset = asset!("/assets/icon/file-local-hover.svg");

const FILE_CLOUD_STILL: Asset = asset!("/assets/icon/file-cloud-still.svg");

const FILE_CLOUD_HOVER: Asset = asset!("/assets/icon/file-cloud-hover.svg");

const COPY_LINK_STILL: Asset = asset!("/assets/icon/copy-link-still.svg");

const COPY_LINK_HOVER: Asset = asset!("/assets/icon/copy-link-hover.svg");

static MANILLA_FOLDER_COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn sleep_ms(ms: i32) {
    let promise = Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });

    let _ = JsFuture::from(promise).await;
}

async fn load_pdf(url: &str) -> Result<(js_sys::Object, usize), JsValue> {
    let window = web_sys::window().unwrap();

    let mut pdfjs_lib = Reflect::get(&window, &"pdfjsLib".into()).unwrap_or(JsValue::UNDEFINED);

    while pdfjs_lib.is_undefined() {
        sleep_ms(50).await;

        pdfjs_lib = Reflect::get(&window, &"pdfjsLib".into())?;
    }

    let worker_opts = Reflect::get(&pdfjs_lib, &"GlobalWorkerOptions".into())?;

    Reflect::set(
        &worker_opts,
        &"workerSrc".into(),
        &"/lib/pdfjs/pdf.worker.mjs".into(),
    )?;

    let config = js_sys::Object::new();

    Reflect::set(&config, &"url".into(), &url.into())?;

    Reflect::set(&config, &"wasmUrl".into(), &"/lib/pdfjs/".into())?;

    let get_doc_fn: js_sys::Function = Reflect::get(&pdfjs_lib, &"getDocument".into())?.into();

    let task = get_doc_fn.call1(&pdfjs_lib, &config)?;

    let promise: Promise = Reflect::get(&task, &"promise".into())?.into();

    let pdf = JsFuture::from(promise).await?;

    let num_pages = Reflect::get(&pdf, &"numPages".into())?.as_f64().unwrap() as usize;

    Ok((pdf.into(), num_pages))
}

async fn render_pdf_page(
    pdf: &js_sys::Object,
    page_num: usize,
    canvas_id: &str,
) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();

    let dpr = window.device_pixel_ratio();

    let get_page_fn: js_sys::Function = Reflect::get(pdf, &"getPage".into())?.into();

    let promise: Promise = get_page_fn
        .call1(pdf, &JsValue::from(page_num as f64))?
        .into();

    let page = JsFuture::from(promise).await?;

    let get_viewport_fn: js_sys::Function = Reflect::get(&page, &"getViewport".into())?.into();

    let logical_args = js_sys::Object::new();

    Reflect::set(&logical_args, &"scale".into(), &JsValue::from(1.0))?;

    let logical_viewport = get_viewport_fn.call1(&page, &logical_args)?;

    let logical_height = Reflect::get(&logical_viewport, &"height".into())?
        .as_f64()
        .unwrap();

    let logical_width = Reflect::get(&logical_viewport, &"width".into())?
        .as_f64()
        .unwrap();

    let render_args = js_sys::Object::new();

    Reflect::set(&render_args, &"scale".into(), &JsValue::from(dpr))?;

    let render_viewport = get_viewport_fn.call1(&page, &render_args)?;

    if let Some(canvas_el) = document.get_element_by_id(canvas_id) {
        let canvas: web_sys::HtmlCanvasElement = canvas_el.clone().dyn_into().unwrap();

        canvas.set_height(
            Reflect::get(&render_viewport, &"height".into())?
                .as_f64()
                .unwrap() as u32,
        );

        canvas.set_width(
            Reflect::get(&render_viewport, &"width".into())?
                .as_f64()
                .unwrap() as u32,
        );

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        let render_task_args = js_sys::Object::new();

        Reflect::set(&render_task_args, &"canvasContext".into(), &context)?;

        Reflect::set(&render_task_args, &"viewport".into(), &render_viewport)?;

        let render_fn: js_sys::Function = Reflect::get(&page, &"render".into())?.into();

        let render_task = render_fn.call1(&page, &render_task_args)?;

        let render_promise: Promise = Reflect::get(&render_task, &"promise".into())?.into();

        JsFuture::from(render_promise).await?;

        // Extract current visual footprint via Reflect with a fallback to logical sizes for hidden elements

        let cw = Reflect::get(&canvas_el, &"clientWidth".into())?
            .as_f64()
            .unwrap_or(0.0);

        let ch = Reflect::get(&canvas_el, &"clientHeight".into())?
            .as_f64()
            .unwrap_or(0.0);

        let visual_width = if cw > 0.0 { cw } else { logical_width };

        let visual_height = if ch > 0.0 { ch } else { logical_height };

        let scale_x = visual_width / logical_width;

        let scale_y = visual_height / logical_height;

        let text_layer_id = format!("text-{}", canvas_id);

        if let Some(text_layer_el) = document.get_element_by_id(&text_layer_id) {
            let text_layer_html: web_sys::HtmlElement = text_layer_el.dyn_into().unwrap();

            let get_content_fn: js_sys::Function =
                Reflect::get(&page, &"getTextContent".into())?.into();

            let content_promise: Promise = get_content_fn.call0(&page)?.into();

            let content = JsFuture::from(content_promise).await?;

            let items = Reflect::get(&content, &"items".into())?.dyn_into::<js_sys::Array>()?;

            let mut inner_html = String::with_capacity(items.length() as usize * 150);

            for i in 0..items.length() {
                let item = items.get(i);

                let text_str = Reflect::get(&item, &"str".into())?
                    .as_string()
                    .unwrap_or_default();

                if text_str.trim().is_empty() {
                    continue;
                }

                let transform =
                    Reflect::get(&item, &"transform".into())?.dyn_into::<js_sys::Array>()?;

                let dir = Reflect::get(&item, &"dir".into())
                    .map(|v| v.as_string().unwrap_or_default())
                    .unwrap_or_default();

                let is_ttb = dir == "ttb";

                let a = transform.get(0).as_f64().unwrap();

                let b = transform.get(1).as_f64().unwrap();

                let c = transform.get(2).as_f64().unwrap();

                let d = transform.get(3).as_f64().unwrap();

                let mut tx = transform.get(4).as_f64().unwrap();

                let ty = transform.get(5).as_f64().unwrap();

                let native_font_height = (a * a + b * b).sqrt().max((c * c + d * d).sqrt());

                // Extract bounds (we only need width for TTB centering)

                let item_width = Reflect::get(&item, &"width".into())?
                    .as_f64()
                    .unwrap_or(native_font_height);

                // item_height is deliberately ignored — it is unreliable for vertical text

                if is_ttb {
                    // PDF.js puts the column center in tx for vertical text

                    tx -= item_width / 2.0;
                }

                let left_pct = (tx / logical_width) * 100.0;

                let top_pct = ((logical_height - ty) / logical_height) * 100.0;

                let font_height_cqh = (native_font_height / logical_height) * 100.0;

                // FIXED: no y_scale_correction, no extra scaling on b/d

                let css_matrix = if is_ttb {
                    format!(
                        "matrix({}, {}, {}, {}, 0, 0)",
                        a / native_font_height,
                        b / native_font_height,
                        c / native_font_height,
                        d / native_font_height
                    )
                } else {
                    format!(
                        "matrix({}, {}, {}, {}, 0, 0)",
                        a / native_font_height,
                        -b / native_font_height,
                        -c / native_font_height,
                        -d / native_font_height
                    )
                };

                let safe_text = text_str.replace("<", "&lt;").replace(">", "&gt;");

                let writing_mode_css = if is_ttb {
                    "writing-mode: vertical-rl; line-height: 1;"
                } else {
                    "line-height: 1;"
                };

                inner_html.push_str(&format!(

                    "<span style=\"position: absolute; left: {:.4}%; top: {:.4}%; transform-origin: 0% 0%; white-space: pre; font-size: {:.4}cqh; font-family: sans-serif; transform: {}; {}\">{}</span>",

                    left_pct, top_pct, font_height_cqh, css_matrix, writing_mode_css, safe_text

                ));
            }

            text_layer_html.set_inner_html(&inner_html);
        }

        let cleanup_fn: js_sys::Function = Reflect::get(&page, &"cleanup".into())?.into();

        let _ = cleanup_fn.call0(&page);
    }

    Ok(())
}

#[component]

pub fn ManillaFolder(file_ref: File) -> Element {
    let internal_source = file_ref.internal_source.clone();

    let external_source_1 = file_ref.external_source.clone();

    let external_source_2 = file_ref.external_source.clone();

    let mut hasher = DefaultHasher::new();

    file_ref.internal_source.hash(&mut hasher);

    let hash_val = hasher.finish() as u16;

    let hash_str = format!("{:04X}", hash_val);

    // CRITICAL FIX 1: Lock the ID generation inside a hook.

    // If you don't do this, every button click triggers a render, generates a new ID,

    // and breaks the DOM targeting completely.

    let folder_id = use_hook(|| MANILLA_FOLDER_COUNTER.fetch_add(1, Ordering::Relaxed));

    let f_id = folder_id.to_string();

    let canvas_id = format!("canvas-{}", f_id);

    let mut current_page = use_signal(|| 1usize);

    let mut total_pages = use_signal(|| 1usize);

    let rendered_pages =
        use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(HashSet::<usize>::new())));

    let pdf_doc = use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(None::<js_sys::Object>)));

    use_effect({
        let pdf_url = internal_source.clone();

        let pdf_doc = pdf_doc.clone();

        move || {
            let url = pdf_url.clone();

            let pdf_doc_clone = pdf_doc.clone();

            spawn(async move {
                if let Ok((pdf, pages)) = load_pdf(&url).await {
                    *pdf_doc_clone.borrow_mut() = Some(pdf);

                    total_pages.set(pages + 1);
                }
            });
        }
    });

    use_effect({
        let pdf_doc = pdf_doc.clone();

        let canvas_id = canvas_id.clone();

        let rendered_pages = rendered_pages.clone();

        move || {
            let page = current_page();

            let total = total_pages();

            let doc_opt = pdf_doc.borrow().clone();

            if let Some(pdf) = doc_opt {
                let mut pages_to_render = vec![page];

                if page > 1 {
                    pages_to_render.push(page - 1);
                }

                if page < total {
                    pages_to_render.push(page + 1);
                }

                // Filter down to only the pages we actually need to render right now

                let needed_pages: Vec<usize> = pages_to_render
                    .into_iter()
                    .filter(|p| !rendered_pages.borrow().contains(p))
                    .collect();

                if needed_pages.is_empty() {
                    return;
                }

                // Optimistically lock them so rapidly clicking doesn't queue 100 requests

                for p in &needed_pages {
                    rendered_pages.borrow_mut().insert(*p);
                }

                let cid_base = canvas_id.clone();

                let rendered_pages_clone = rendered_pages.clone();

                // CRITICAL FIX 2: Sequential Rendering.

                // PDF.js will crash and burn if you tell it to render the exact same page

                // in concurrent async tasks. We loop sequentially inside one task instead.

                spawn(async move {
                    let window = web_sys::window().unwrap();

                    let document = window.document().unwrap();

                    for p in needed_pages {
                        let cid = format!("{}-{}", cid_base, p);

                        let target_pdf_page = if p == 1 { 1 } else { p - 1 };

                        let mut retries = 0;

                        while document.get_element_by_id(&cid).is_none() && retries < 20 {
                            sleep_ms(50).await;

                            retries += 1;
                        }

                        if document.get_element_by_id(&cid).is_some() {
                            let result = render_pdf_page(&pdf, target_pdf_page, &cid).await;

                            if result.is_err() {
                                // If render fails, unlock it so we can try again

                                rendered_pages_clone.borrow_mut().remove(&p);
                            }
                        } else {
                            rendered_pages_clone.borrow_mut().remove(&p);
                        }
                    }
                });
            }
        }
    });

    let is_cover = current_page() == 1;

    rsx! {

    div { class: "source-counter-area",

    div { class: "source-deactivate-button-area" }

    div { class: "source-counter",

    span {

    class: if is_cover { "source-button disabled" } else { "source-button" },

    onclick: move |_| if current_page() > 1 { current_page -= 1 },

    "<"

    }

    span { "{current_page} of {total_pages}" }

    span {

    class: if current_page() >= total_pages() { "source-button disabled" } else { "source-button" },

    onclick: move |_| if current_page() < total_pages() { current_page += 1 },

    ">"

    }

    }

    }



    div { id: format!("manilla-folder-{f_id}"), class: "manilla-folder",

    div { class: "manilla-folder-cover folder-cover-back",

    img { src: MANILLA_FOLDER_BACK, draggable: false },

    div { class: "folder-tab-text-area tab-text-area-name",

    span { class: "tab-text", dangerous_inner_html: "{file_ref.source_citation}" }

    }

    div { class: "folder-tab-text-area tab-text-area-hash",

    span { class: "tab-text", "{hash_str}" }

    }

    }



    for p in 1..=total_pages() {

    div {

    key: "{f_id}-page-{p}",

    class: "pdf-single-page-container",

    // Use visibility: hidden. It removes the page from view without collapsing dimensions to 0x0.

    style: if p != current_page() {

    "visibility: hidden; pointer-events: none;"

    } else {

    "visibility: visible;"

    },

    canvas { id: "{canvas_id}-{p}", class: "pdf-single-canvas" }

    div { id: "text-{canvas_id}-{p}", class: "textLayer" }

    }

    }



    div {

    class: "manilla-folder-cover folder-cover-face",

    style: if !is_cover { "display: none;" } else { "display: block;" },

    img { src: MANILLA_FOLDER_FACE, draggable: false }

    }



    div { class: "file-button-area",

    a { class: "bipolar-button", href: "{internal_source}", target: "_blank", style: "--button-image-still: url({FILE_LOCAL_STILL}); --button-image-hover: url({FILE_LOCAL_HOVER});" }

    a { class: "bipolar-button", href: "{external_source_1}", target: "_blank", style: "--button-image-still: url({FILE_CLOUD_STILL}); --button-image-hover: url({FILE_CLOUD_HOVER});" }

    a { class: "bipolar-button", href: "{external_source_2}", target: "_blank", style: "--button-image-still: url({COPY_LINK_STILL}); --button-image-hover: url({COPY_LINK_HOVER});" }

    }

    }

    }
}
