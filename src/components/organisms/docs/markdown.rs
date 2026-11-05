//! Markdown rendering for the docs pages.
//!
//! The articles are embedded at build time (`include_str!`) and parsed here
//! with [`pulldown_cmark`] — pure Rust, so it runs in the static wasm client.
//! A small recursive-descent renderer walks the event stream and emits rsx
//! elements styled to match the site theme, instead of relying on an HTML
//! push renderer.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::Closure, JsCast};

use dioxus::prelude::*;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};

use crate::components::scroll_to_id;

/// Render a markdown document as styled Dioxus elements.
///
/// Supports headings, paragraphs, emphasis/strong/strikethrough, inline code,
/// fenced code blocks (with a copy button), lists, task lists, tables, block
/// quotes, rules, links, and images. Raw HTML in the source is dropped.
#[component]
pub fn MarkdownView(markdown: &'static str) -> Element {
    let options =
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    let events: Vec<Event<'_>> = Parser::new_ext(markdown, options).collect();

    let mut cursor = 0usize;
    let mut blocks: Vec<Element> = Vec::new();
    // parse_blocks stops at an End event (the close of a nested container);
    // loop until the stream is exhausted so a stray End can never truncate
    // the rest of the document.
    while cursor < events.len() {
        blocks.extend(parse_blocks(&events, &mut cursor));
    }

    rsx! {
        div {
            class: "markdown-view",
            for block in blocks {
                {block}
            }
        }
    }
}

/// A fenced code block with a language badge and a copy-to-clipboard button.
#[component]
fn CodeBlock(lang: String, code: String) -> Element {
    let mut copied = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    let clipboard_code = code.clone();

    let onclick = move |_| {
        copied.set(true);
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&clipboard_code);
                // Flip the label back after a moment.
                let mut copied = copied;
                let callback = Closure::<dyn FnMut()>::new(move || copied.set(false));
                if window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        2000,
                    )
                    .is_ok()
                {
                    // Keep the timer alive until it fires (tiny one-off leak).
                    callback.forget();
                }
            }
        }
    };

    rsx! {
        div {
            class: "my-6 overflow-hidden rounded-lg border border-white/10",
            div {
                class: "flex items-center justify-between bg-white/5 px-4 py-2",
                span {
                    class: "text-xs font-medium uppercase tracking-wider text-gray-500",
                    if lang.is_empty() { "code" } else { "{lang}" }
                }
                button {
                    class: "flex items-center gap-1.5 text-xs text-gray-400 transition-colors hover:text-white",
                    "aria-label": "Copy code to clipboard",
                    onclick,
                    i {
                        class: if copied() { "fa-solid fa-check text-green-400" } else { "fa-regular fa-copy" },
                        "aria-hidden": "true",
                    }
                    if copied() { "Copied!" } else { "Copy" }
                }
            }
            pre {
                class: "overflow-x-auto bg-black/50 p-4",
                code { class: "whitespace-pre font-mono text-sm leading-relaxed text-purple-100/90", "{code}" }
            }
        }
    }
}

/// Render block-level events until the enclosing container closes (its `End`
/// event is consumed) or the stream runs out.
fn parse_blocks(events: &[Event<'_>], cursor: &mut usize) -> Vec<Element> {
    let mut blocks: Vec<Element> = Vec::new();

    while *cursor < events.len() {
        match &events[*cursor] {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    *cursor += 1;
                    let inline = parse_inline(events, cursor);
                    blocks.push(rsx! {
                        p { class: "my-4 leading-relaxed text-gray-300", for element in inline { {element} } }
                    });
                }
                Tag::Heading { level, .. } => {
                    // Slug the heading text into an anchor id so in-article
                    // links (`[First launch](#first-launch)`) can scroll.
                    let id = slugify(&peek_text_until_end(events, *cursor));
                    *cursor += 1;
                    let inline = parse_inline(events, cursor);
                    blocks.push(render_heading(*level, inline, id));
                }
                Tag::CodeBlock(kind) => {
                    *cursor += 1;
                    let mut code = String::new();
                    while *cursor < events.len() {
                        match &events[*cursor] {
                            Event::Text(chunk) => {
                                code.push_str(chunk);
                                *cursor += 1;
                            }
                            Event::End(_) => {
                                *cursor += 1;
                                break;
                            }
                            _ => *cursor += 1,
                        }
                    }
                    let lang = match kind {
                        CodeBlockKind::Fenced(info) => info
                            .split([',', ' '])
                            .next()
                            .unwrap_or_default()
                            .to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    blocks.push(rsx! { CodeBlock { lang, code } });
                }
                Tag::List(numbered) => {
                    *cursor += 1;
                    let items = parse_list_items(events, cursor);
                    blocks.push(match numbered {
                        Some(_) => rsx! {
                            ol {
                                class: "my-4 list-decimal space-y-2 pl-6 marker:font-semibold marker:text-purple-400",
                                for item in items { {item} }
                            }
                        },
                        None => rsx! {
                            ul {
                                class: "my-4 list-disc space-y-2 pl-6 marker:text-purple-400",
                                for item in items { {item} }
                            }
                        },
                    });
                }
                Tag::BlockQuote(_) => {
                    *cursor += 1;
                    let inner = parse_blocks(events, cursor);
                    blocks.push(rsx! {
                        blockquote {
                            class: "my-6 border-l-2 border-purple-500/60 bg-white/[0.03] py-1 pl-4 text-gray-400",
                            for element in inner { {element} }
                        }
                    });
                }
                Tag::Table(alignments) => {
                    let alignments = alignments.clone();
                    *cursor += 1;

                    let mut head: Vec<Element> = Vec::new();
                    if matches!(events.get(*cursor), Some(Event::Start(Tag::TableHead))) {
                        *cursor += 1;
                        head = parse_table_cells(events, cursor, &alignments, true);
                    }

                    let mut rows: Vec<Element> = Vec::new();
                    while *cursor < events.len() {
                        match &events[*cursor] {
                            Event::Start(Tag::TableRow) => {
                                *cursor += 1;
                                let cells = parse_table_cells(events, cursor, &alignments, false);
                                rows.push(rsx! {
                                    tr { class: "transition-colors hover:bg-white/[0.03]", for cell in cells { {cell} } }
                                });
                            }
                            Event::End(_) => {
                                *cursor += 1;
                                break;
                            }
                            _ => *cursor += 1,
                        }
                    }

                    blocks.push(rsx! {
                        div {
                            class: "my-6 overflow-x-auto rounded-lg border border-white/10",
                            table {
                                class: "w-full border-collapse text-sm",
                                thead { for cell in head { {cell} } }
                                tbody { for row in rows { {row} } }
                            }
                        }
                    });
                }
                // Raw HTML is dropped, not rendered.
                Tag::HtmlBlock => {
                    *cursor += 1;
                    while *cursor < events.len() {
                        match &events[*cursor] {
                            Event::End(_) => {
                                *cursor += 1;
                                break;
                            }
                            _ => *cursor += 1,
                        }
                    }
                }
                _ => *cursor += 1,
            },
            Event::Rule => {
                *cursor += 1;
                blocks.push(rsx! { hr { class: "my-8 border-white/10" } });
            }
            // Close of the enclosing container.
            Event::End(_) => {
                *cursor += 1;
                break;
            }
            _ => *cursor += 1,
        }
    }

    blocks
}

/// Render inline events until the enclosing container closes (its `End`
/// event is consumed).
fn parse_inline(events: &[Event<'_>], cursor: &mut usize) -> Vec<Element> {
    let mut out: Vec<Element> = Vec::new();
    let mut text = String::new();

    while *cursor < events.len() {
        match &events[*cursor] {
            Event::End(_) => {
                *cursor += 1;
                break;
            }
            Event::Text(chunk) => {
                text.push_str(chunk);
                *cursor += 1;
            }
            Event::Code(chunk) => {
                flush_text(&mut text, &mut out);
                let code = chunk.clone();
                out.push(rsx! {
                    code { class: "rounded bg-white/10 px-1.5 py-0.5 font-mono text-[0.9em] text-pink-300", "{code}" }
                });
                *cursor += 1;
            }
            Event::SoftBreak => {
                text.push(' ');
                *cursor += 1;
            }
            Event::HardBreak => {
                flush_text(&mut text, &mut out);
                out.push(rsx! { br {} });
                *cursor += 1;
            }
            Event::Start(Tag::Emphasis) => {
                *cursor += 1;
                let inner = parse_inline(events, cursor);
                flush_text(&mut text, &mut out);
                out.push(rsx! { em { class: "italic", for element in inner { {element} } } });
            }
            Event::Start(Tag::Strong) => {
                *cursor += 1;
                let inner = parse_inline(events, cursor);
                flush_text(&mut text, &mut out);
                out.push(rsx! { strong { class: "font-semibold text-white", for element in inner { {element} } } });
            }
            Event::Start(Tag::Strikethrough) => {
                *cursor += 1;
                let inner = parse_inline(events, cursor);
                flush_text(&mut text, &mut out);
                out.push(rsx! { del { class: "text-gray-500 line-through", for element in inner { {element} } } });
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                *cursor += 1;
                let inner = parse_inline(events, cursor);
                flush_text(&mut text, &mut out);
                let href = dest_url.to_string();
                let external = href.starts_with("http://") || href.starts_with("https://");
                // In-article anchors scroll instantly — the browser's native
                // hash scroll uses the CSS smooth behavior, which can be
                // dropped entirely, leaving the user at the top.
                let anchor = (!external && href.len() > 1 && href.starts_with('#'))
                    .then(|| href[1..].to_string());
                let class = "font-medium text-purple-400 underline decoration-purple-400/40 underline-offset-2 transition-colors hover:text-purple-300";
                match anchor {
                    Some(anchor) => out.push(rsx! {
                        a {
                            href,
                            class,
                            onclick: move |event: MouseEvent| {
                                event.prevent_default();
                                scroll_to_id(&anchor);
                            },
                            for element in inner { {element} }
                        }
                    }),
                    None => out.push(rsx! {
                        a {
                            href,
                            target: if external { "_blank" },
                            rel: if external { "noopener noreferrer" },
                            class,
                            for element in inner { {element} }
                        }
                    }),
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                *cursor += 1;
                let alt = collect_image_alt(events, cursor);
                flush_text(&mut text, &mut out);
                let src = dest_url.to_string();
                out.push(rsx! {
                    img { src, alt, class: "my-6 max-w-full rounded-xl border border-white/10" }
                });
            }
            // A nested block inside inline content (e.g. a list inside a
            // tight list item): hand it back to the block renderer so it
            // isn't mangled.
            Event::Start(
                Tag::List(_) | Tag::BlockQuote(_) | Tag::CodeBlock(_) | Tag::Table(_),
            ) => {
                out.extend(parse_blocks(events, cursor));
            }
            // Raw inline HTML is dropped, not rendered.
            Event::InlineHtml(_) => *cursor += 1,
            _ => *cursor += 1,
        }
    }

    flush_text(&mut text, &mut out);
    out
}

/// Render the items of one list. Consumes events up to and including the
/// list's `End`.
fn parse_list_items(events: &[Event<'_>], cursor: &mut usize) -> Vec<Element> {
    let mut items: Vec<Element> = Vec::new();

    while *cursor < events.len() {
        match &events[*cursor] {
            Event::Start(Tag::Item) => {
                *cursor += 1;
                // Task lists: the marker event comes first inside the item.
                let mut checked: Option<bool> = None;
                if let Some(Event::TaskListMarker(state)) = events.get(*cursor) {
                    checked = Some(*state);
                    *cursor += 1;
                }
                // Tight lists keep inline events bare; loose lists wrap them
                // in paragraphs (possibly with nested blocks).
                let tight = !matches!(events.get(*cursor), Some(Event::Start(Tag::Paragraph)));
                let content = if tight {
                    parse_inline(events, cursor)
                } else {
                    parse_blocks(events, cursor)
                };
                match checked {
                    Some(state) => items.push(rsx! {
                        li {
                            class: "list-none",
                            i {
                                class: if state {
                                    "fa-solid fa-square-check mr-2 text-purple-400"
                                } else {
                                    "fa-regular fa-square mr-2 text-gray-600"
                                },
                                "aria-hidden": "true",
                            }
                            for element in content { {element} }
                        }
                    }),
                    None => items.push(rsx! {
                        li { class: "pl-1 text-gray-300", for element in content { {element} } }
                    }),
                }
            }
            Event::End(_) => {
                *cursor += 1;
                break;
            }
            _ => *cursor += 1,
        }
    }

    items
}

/// Render the cells of a table head or row. Consumes events up to and
/// including the row's `End`.
fn parse_table_cells(
    events: &[Event<'_>],
    cursor: &mut usize,
    alignments: &[Alignment],
    head: bool,
) -> Vec<Element> {
    let mut cells: Vec<Element> = Vec::new();

    while *cursor < events.len() {
        match &events[*cursor] {
            Event::Start(Tag::TableCell) => {
                let align = alignments
                    .get(cells.len())
                    .copied()
                    .unwrap_or(Alignment::None);
                let align_class = match align {
                    Alignment::Left => "text-left",
                    Alignment::Center => "text-center",
                    Alignment::Right => "text-right",
                    Alignment::None => "text-left",
                };
                *cursor += 1;
                let content = parse_inline(events, cursor);
                cells.push(if head {
                    rsx! {
                        th { class: "bg-white/5 px-4 py-2.5 font-semibold text-white {align_class}", for element in content { {element} } }
                    }
                } else {
                    rsx! {
                        td { class: "border-t border-white/10 px-4 py-2.5 text-gray-300 {align_class}", for element in content { {element} } }
                    }
                });
            }
            Event::End(_) => {
                *cursor += 1;
                break;
            }
            _ => *cursor += 1,
        }
    }

    cells
}

fn render_heading(level: HeadingLevel, inline: Vec<Element>, id: String) -> Element {
    let id = if id.is_empty() { None } else { Some(id) };
    match level {
        HeadingLevel::H1 => rsx! {
            h1 { id, class: "mb-6 mt-2 border-b border-white/10 pb-3 text-3xl font-bold text-white sm:text-4xl", for element in inline { {element} } }
        },
        HeadingLevel::H2 => rsx! {
            h2 { id, class: "mb-4 mt-10 text-2xl font-semibold text-white", for element in inline { {element} } }
        },
        HeadingLevel::H3 => rsx! {
            h3 { id, class: "mb-3 mt-8 text-xl font-semibold text-purple-300", for element in inline { {element} } }
        },
        HeadingLevel::H4 => rsx! {
            h4 { id, class: "mb-2 mt-6 text-lg font-semibold text-gray-100", for element in inline { {element} } }
        },
        HeadingLevel::H5 => rsx! {
            h5 { id, class: "mb-2 mt-4 font-semibold text-gray-200", for element in inline { {element} } }
        },
        HeadingLevel::H6 => rsx! {
            h6 { id, class: "mb-2 mt-4 text-sm font-semibold text-gray-400", for element in inline { {element} } }
        },
    }
}

/// Collect the plain text of the current inline run (up to its `End` event)
/// without consuming anything — used to derive heading anchor ids.
fn peek_text_until_end(events: &[Event<'_>], start: usize) -> String {
    let mut out = String::new();
    for event in &events[start..] {
        match event {
            Event::Text(chunk) | Event::Code(chunk) => out.push_str(chunk),
            Event::SoftBreak => out.push(' '),
            Event::End(_) => break,
            _ => {}
        }
    }
    out
}

/// GitHub-style anchor slug: lowercase, keep `[a-z0-9-]`, spaces become `-`.
fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| match c {
            'a'..='z' | '0'..='9' | '-' => Some(c),
            ' ' => Some('-'),
            _ => None,
        })
        .collect()
}

/// Accumulate a run of plain text and emit it as one text node.
fn flush_text(text: &mut String, out: &mut Vec<Element>) {
    if !text.is_empty() {
        let chunk = std::mem::take(text);
        out.push(rsx! { "{chunk}" });
    }
}

/// Collect the alt text of an image. Consumes events up to and including the
/// image's `End`.
fn collect_image_alt(events: &[Event<'_>], cursor: &mut usize) -> String {
    let mut alt = String::new();
    while *cursor < events.len() {
        match &events[*cursor] {
            Event::End(_) => {
                *cursor += 1;
                break;
            }
            Event::Text(chunk) => {
                alt.push_str(chunk);
                *cursor += 1;
            }
            _ => *cursor += 1,
        }
    }
    alt
}
