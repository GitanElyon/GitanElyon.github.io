use crate::data::projects::get_projects;
use crate::views::home::{clear_theme_lock, hex_to_hue, set_theme_lock};
use dioxus::prelude::*;

const SHOWCASE_DURATION_MS: f64 = 8000.0; // 8 seconds per featured project
const TICK_MS: f64 = 50.0;

#[component]
pub fn Projects() -> Element {
    let projects = get_projects();
    let mut selected = use_signal(|| None::<String>);

    let featured: Vec<_> = projects.iter().filter(|p| p.is_featured).cloned().collect();
    let non_featured: Vec<_> = projects.iter().filter(|p| !p.is_featured).cloned().collect();

    let featured_count = featured.len();
    let mut showcase_idx = use_signal(|| 0usize);
    let mut timer_pct = use_signal(|| 0.0f64);
    let mut paused = use_signal(|| false);

    // Auto-rotate the showcase using web_sys setInterval
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use std::rc::Rc;
            use std::cell::Cell;

            let tick_count = Rc::new(Cell::new(0u64));
            let total_ticks = (SHOWCASE_DURATION_MS / TICK_MS) as u64;

            let closure = Closure::<dyn FnMut()>::new(move || {
                if paused() {
                    return;
                }
                let t = tick_count.get() + 1;
                tick_count.set(t);
                let pct = (t as f64 / total_ticks as f64) * 100.0;
                timer_pct.set(pct.min(100.0));
                if t >= total_ticks {
                    tick_count.set(0);
                    timer_pct.set(0.0);
                    let next = (showcase_idx() + 1) % featured_count;
                    showcase_idx.set(next);
                }
            });

            if let Some(win) = web_sys::window() {
                let _ = win.set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    TICK_MS as i32,
                );
            }
            closure.forget();
        });
    }

    let open = |id: &'static str, glow: &'static str| {
        let s = id.to_string();
        let color = glow.to_string();
        move |_| {
            selected.set(Some(s.clone()));
            paused.set(true);
            timer_pct.set(0.0);
            set_theme_lock(hex_to_hue(&color));
            // Scroll to showcase
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(win) = web_sys::window() {
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.query_selector(".showcase-wrapper").ok().flatten() {
                            el.scroll_into_view();
                        }
                    }
                }
            }
        }
    };

    // Determine current showcase project — if a tile was clicked and it's a non-featured project,
    // show it in the showcase; otherwise show the auto-rotating featured project.
    let showcase_project = if let Some(ref sel_id) = selected() {
        projects.iter().find(|p| p.id == *sel_id).cloned()
    } else {
        featured.get(showcase_idx()).cloned()
    };

    rsx! {
        section { id: "projects",
            div { class: "container",
                h1 { class: "page-title", "My Projects" }

                // ── Showcase ──
                if let Some(project) = &showcase_project {
                    {
                        let is_selected = selected().is_some();
                        rsx! {
                            div {
                                class: "showcase-wrapper",
                                style: "--glow-color: {project.glow_color}",
                                onmouseenter: move |_| paused.set(true),
                                onmouseleave: move |_| {
                                    if selected().is_none() {
                                        paused.set(false);
                                    }
                                },
                                div { class: "showcase-content",
                                    // Close button when a project is focused
                                    if is_selected {
                                        button {
                                            class: "showcase-close-btn",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                selected.set(None);
                                                paused.set(false);
                                                timer_pct.set(0.0);
                                                clear_theme_lock();
                                            },
                                            "✕"
                                        }
                                    }
                                    h3 { "{project.name}" }
                                    // Show detailed_description when a project is selected, otherwise the short description
                                    if is_selected {
                                        for para in &project.detailed_description {
                                            p { class: "detail", "{para}" }
                                        }
                                    } else {
                                        p { "{project.description}" }
                                    }
                                    div { class: "tags",
                                        for tech in &project.technologies {
                                            span { "{tech}" }
                                        }
                                    }
                                    div { class: "showcase-links",
                                        if let Some(url) = project.github_url {
                                            a {
                                                href: "{url}",
                                                target: "_blank",
                                                class: "link",
                                                onclick: |e: Event<MouseData>| e.stop_propagation(),
                                                "GitHub ↗"
                                            }
                                        }
                                        if let Some(url) = project.demo_url {
                                            a {
                                                href: "{url}",
                                                target: "_blank",
                                                class: "link",
                                                onclick: |e: Event<MouseData>| e.stop_propagation(),
                                                "Live Demo ↗"
                                            }
                                        }
                                    }
                                    if project.private {
                                        p { class: "note", "Private Repository" }
                                    }
                                }
                                // Timer bar (only when auto-rotating, not when user selected)
                                if !is_selected {
                                    div { class: "showcase-timer", style: "width: {timer_pct()}%" }
                                }
                            }
                        }
                    }

                    // Dots below showcase (only when no project is manually selected)
                    if selected().is_none() {
                        div { class: "showcase-dots",
                            for (i , fp) in featured.iter().enumerate() {
                                button {
                                    class: if i == showcase_idx() { "dot active" } else { "dot" },
                                    onclick: move |_| {
                                        showcase_idx.set(i);
                                        timer_pct.set(0.0);
                                    },
                                    title: "{fp.name}",
                                }
                            }
                        }
                    }
                }

                // ── All other projects ──
                p { class: "hint", "Click any card to see details above" }
                div { class: "projects-list",
                    for project in projects {
                        div {
                            class: if selected() == Some(project.id.to_string()) { "project-tile selected" } else { "project-tile" },
                            style: "--glow-color: {project.glow_color}",
                            onclick: open(project.id, project.glow_color),
                            h3 { "{project.name}" }
                            p { class: "brief", "{project.brief}" }
                        }
                    }
                }
            }
        }
    }
}
