use crate::components::ContactForm;
use crate::data::profile::get_profile_data;
use crate::data::projects::get_projects;
use crate::data::skills::{get_about_languages, get_skill_sections};
use dioxus::prelude::*;

// ── Theme helpers (minimal Rust, heavy CSS) ──────────────────────────────

/// Convert a hex color like "#6366f1" to its hue component (0–360).
pub fn hex_to_hue(hex: &str) -> f64 {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return 210.0;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    if delta < 1e-6 {
        return 0.0;
    }
    let h = if (max - r).abs() < 1e-6 {
        60.0 * (((g - b) / delta) % 6.0)
    } else if (max - g).abs() < 1e-6 {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    (h + 360.0) % 360.0
}

/// Lock the opal color system to a specific hue.
/// Sets a single CSS variable and a class — all transitions done in CSS.
pub fn set_theme_lock(hue: f64) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.document_element() {
                    let html = el.unchecked_into::<web_sys::HtmlElement>();
                    let _ = html.dataset().set("locked", "");
                    let _ = html.style().set_property("--hue", &format!("{hue}"));
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = hue;
    }
}

/// Unlock theme, resume slow CSS cycling.
pub fn clear_theme_lock() {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(win) = web_sys::window() {
            if let Some(doc) = win.document() {
                if let Some(el) = doc.document_element() {
                    let html = el.unchecked_into::<web_sys::HtmlElement>();
                    let _ = html.remove_attribute("data-locked");
                    let _ = html.style().remove_property("--hue");
                }
            }
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn skill_color(name: &str) -> &'static str {
    match name {
        "Rust" => "#DEA584",
        "Go" => "#00ADD8",
        "Python" => "#3776AB",
        "JavaScript" | "JS/TS" => "#F7DF1E",
        "TypeScript" => "#3178C6",
        "Java" => "#ED8B00",
        "HTML" | "HTML/CSS" => "#E34F26",
        "CSS" => "#1572B6",
        "C" | "C/C++" => "#A8B9CC",
        "C++" => "#00599C",
        "C#" => "#512BD4",
        "Lua" => "#2C2D72",
        "Tauri" => "#24C8DB",
        "Bun.js" => "#FBF0DF",
        "Dioxus" => "#EBB35D",
        "Node.js" => "#339933",
        "Svelte" => "#FF3E00",
        "Vue" => "#42B883",
        "React" => "#61DAFB",
        "Electron" => "#47848F",
        "Linux" => "#FCC624",
        "Git" => "#F05032",
        "Nix" => "#5277C3",
        "Docker" => "#2496ED",
        "Adobe" => "#FF0000",
        "Figma" => "#F24E1E",
        "PostgreSQL" => "#4169E1",
        "Solr" => "#D9411E",
        "SurrealDB" => "#FF00A0",
        "Cloudflare" => "#F38020",
        _ => "#888888",
    }
}

const HEADSHOT: Asset = asset!("/assets/profile.webp");

// ── Home component ──────────────────────────────────────────────────────

#[component]
pub fn Home() -> Element {
    let sections = get_skill_sections();
    let projects = get_projects();
    let languages = get_about_languages();
    // Single source of truth: career timeline + contact details mirror the profile data.
    let profile = get_profile_data();
    let career = profile.experience.clone();

    let mut carousel_idx = use_signal(|| 0usize);
    let mut selected_skill = use_signal(|| None::<String>);
    let mut expanded_lang = use_signal(|| None::<String>);
    let mut expanded_project = use_signal(|| None::<String>);

    let featured: Vec<_> = projects.iter().filter(|p| p.is_featured).collect();

    use_future(move || async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(500).await;
            // Skip all DOM work unless something is actually expanded —
            // otherwise this would wake up and query the DOM forever.
            if expanded_project().is_none()
                && expanded_lang().is_none()
                && selected_skill().is_none()
            {
                continue;
            }
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(win) = web_sys::window() {
                    let h = win.inner_height().unwrap().as_f64().unwrap();
                    if let Some(doc) = win.document() {
                        if let Some(el) = doc.query_selector(".expanded").ok().flatten() {
                            let rect = el.get_bounding_client_rect();
                            if rect.bottom() < 0.0 || rect.top() > h {
                                expanded_project.set(None);
                                clear_theme_lock();
                            }
                        }
                        if let Some(el) = doc.query_selector(".lang-row.active").ok().flatten() {
                            let rect = el.get_bounding_client_rect();
                            if rect.bottom() < 0.0 || rect.top() > h {
                                expanded_lang.set(None);
                                clear_theme_lock();
                            }
                        }
                        if let Some(el) = doc.query_selector(".skill-icon.active").ok().flatten() {
                            let rect = el.get_bounding_client_rect();
                            if rect.bottom() < 0.0 || rect.top() > h {
                                selected_skill.set(None);
                                clear_theme_lock();
                            }
                        }
                    }
                }
            }
            // Non-wasm targets (SSR/tests): the sleep at the top of the loop
            // is the whole body; there is no DOM to collapse-check.
        }
    });

    // Toggle featured project and lock/unlock theme
    let toggle_project = |id: &'static str, glow: &'static str| {
        let id_string = id.to_string();
        let color = glow.to_string();
        move |_| {
            if expanded_project() == Some(id_string.clone()) {
                expanded_project.set(None);
                clear_theme_lock();
            } else {
                expanded_project.set(Some(id_string.clone()));
                set_theme_lock(hex_to_hue(&color));
            }
        }
    };

    rsx! {
        document::Title { "Gitan Elyon — Systems Engineer" }
        // ── Hero ─────────────────────────────────────────────────
        section { id: "hero",
            div { class: "hero-content",
                div { class: "hero-text",
                    h1 {
                        "Hi, I'm "
                        span { class: "hero-name",
                            span { class: "name-line", "Gitan" }
                            " "
                            span { class: "name-line", "Elyon" }
                        }
                    }
                    h2 { "Systems Engineer" }
                    p {
                        "I build high‑performance applications with modern tools. Passionate about solving real problems using cutting edge technologies."
                    }
                    div { class: "hero-buttons",
                        Link { to: "/projects", class: "btn btn-primary", "View My Work" }
                        Link { to: "/contact", class: "btn btn-secondary", "Get in Touch" }
                    }
                }
                div { class: "hero-image",
                    img {
                        src: HEADSHOT,
                        alt: "Portrait of Gitan Elyon",
                        width: "560",
                        height: "560",
                        decoding: "async",
                    }
                }
            }
        }

        // ── About ────────────────────────────────────────────────
        section { id: "home-about",
            h2 { class: "section-title", "About Me" }
            div { class: "container",
                div { class: "about-grid",
                    // General card
                    div { class: "glass",
                        h3 { "Who I Am" }
                        p {
                            "A Systems Engineer building high-performance backend and security infrastructure with Rust and Go. Currently at Ridgeback Network Defence — previously NexSys, Mind Over Machines, and Gulp Data."
                        }
                    }
                    // Work card
                    div { class: "glass",
                        h3 { "What I Do" }
                        ul {
                            li { "Network security tooling & access-control systems" }
                            li { "Low-latency backends & LLM-driven search" }
                            li { "Cross-platform desktop apps (Tauri, Dioxus)" }
                            li { "Reproducible builds & dev environments with Nix" }
                            li { "Automation & CI/CD pipelines" }
                            li { "Open source: Qst, Minos, Papyr" }
                        }
                    }
                    // Languages card
                    div { class: "glass",
                        h3 { "Languages" }
                        for lang in &languages {
                            div {
                                class: if expanded_lang() == Some(lang.name.to_string()) { "lang-row active" } else { "lang-row" },
                                onclick: {
                                    let name = lang.name.to_string();
                                    let color = lang.color.to_string();
                                    move |_| {
                                        if expanded_lang() == Some(name.clone()) {
                                            expanded_lang.set(None);
                                            clear_theme_lock();
                                        } else {
                                            expanded_lang.set(Some(name.clone()));
                                            set_theme_lock(hex_to_hue(&color));
                                        }
                                    }
                                },
                                span { style: "display:flex;align-items:center;gap:0.5rem;",
                                    span {
                                        class: "dot",
                                        style: "background:{lang.color}",
                                    }
                                    "{lang.name}"
                                }
                                span { style: "color:var(--c-text-muted);font-size:0.85rem;",
                                    "{lang.years}"
                                }
                            }
                            if expanded_lang() == Some(lang.name.to_string()) {
                                div { class: "lang-expanded",
                                    h4 { "{lang.name}" }
                                    p { "{lang.long}" }
                                    div { class: "tags",
                                        for tool in &lang.tools {
                                            span { "{tool}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Skills ───────────────────────────────────────────────
        section { id: "home-skills",
            h2 { class: "section-title", "Skills" }
            p { class: "hint", "Hours are self-tracked estimates" }
            div { class: "carousel-wrapper",
                {
                    let section = &sections[carousel_idx()];
                    rsx! {
                        div { class: "glass skills-glass",
                            // Tabs
                            div { class: "skills-tabs",
                                for (i , sec) in sections.iter().enumerate() {
                                    button {
                                        class: if i == carousel_idx() { "tab active" } else { "tab" },
                                        onclick: move |_| {
                                            carousel_idx.set(i);
                                            selected_skill.set(None);
                                            clear_theme_lock();
                                        },
                                        "{sec.0}"
                                    }
                                }
                            }

                            // Skills bars — icon is bound to each matching bar
                            div { class: "bar-graph",
                                {
                                    let max_h = section.1.iter().map(|s| s.hours).max().unwrap_or(1) as f64;
                                    rsx! {
                                        for skill in section.1.iter() {
                                            {
                                                let pct = (skill.hours as f64 / max_h * 100.0).min(100.0);
                                                let name = skill.name.to_string();
                                                let color = skill_color(skill.name).to_string();
                                                rsx! {
                                                    div { class: "bar-item",
                                                        div { class: "skill-icon-wrap",
                                                            div {
                                                                class: if selected_skill() == Some(name.clone()) { "skill-icon active" } else { "skill-icon" },
                                                                onclick: {
                                                                    let n = name.clone();
                                                                    let c = color.clone();
                                                                    move |_| {
                                                                        if selected_skill() == Some(n.clone()) {
                                                                            selected_skill.set(None);
                                                                            clear_theme_lock();
                                                                        } else {
                                                                            selected_skill.set(Some(n.clone()));
                                                                            set_theme_lock(hex_to_hue(&c));
                                                                        }
                                                                    }
                                                                },
                                                                img {
                                                                    src: skill.icon,
                                                                    alt: "{skill.name} icon",
                                                                    width: "48",
                                                                    height: "48",
                                                                    loading: "lazy",
                                                                    decoding: "async",
                                                                }
                                                            }
                                                        }
                                                        div {
                                                            class: "bar-wrapper",
                                                            onclick: {
                                                                let n = name.clone();
                                                                let c = color.clone();
                                                                move |_| {
                                                                    if selected_skill() == Some(n.clone()) {
                                                                        selected_skill.set(None);
                                                                        clear_theme_lock();
                                                                    } else {
                                                                        selected_skill.set(Some(n.clone()));
                                                                        set_theme_lock(hex_to_hue(&c));
                                                                    }
                                                                }
                                                            },
                                                            div {
                                                                class: "bar-fill",
                                                                style: "--bar-pct:{pct}%;background:linear-gradient(to top,{color},color-mix(in srgb,{color} 60%,white))",
                                                            }
                                                            span { class: "bar-hours", "{skill.hours}h" }
                                                        }
                                                        span { class: "bar-label", "{skill.name}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Selected skill stats
                            if let Some(ref name) = selected_skill() {
                                if let Some(skill) = section.1.iter().find(|s| s.name == name) {
                                    div { class: "skills-stats",
                                        h4 { "{skill.name}" }
                                        p { "Hours: {skill.hours}" }
                                        p { "Projects:" }
                                        div { class: "tags",
                                            for proj in &skill.projects {
                                                span { "{proj}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Dots
                div { class: "dots",
                    for (i , _) in sections.iter().enumerate() {
                        span {
                            class: if i == carousel_idx() { "dot active" } else { "dot" },
                            onclick: move |_| {
                                carousel_idx.set(i);
                                selected_skill.set(None);
                                clear_theme_lock();
                            },
                        }
                    }
                }
            }
        }

        // ── Featured Projects ────────────────────────────────────
        section { id: "home-projects",
            h2 { class: "section-title", "Featured Projects" }
            div { class: "container",
                p { class: "hint", "Click a card to expand" }
                div { class: "projects-row",
                    for project in featured.iter() {
                        {
                            let is_expanded = expanded_project() == Some(project.id.to_string());
                            let any_expanded = expanded_project().is_some();
                            let card_cls = if is_expanded {
                                "glass featured-card expanded"
                            } else if any_expanded {
                                "glass featured-card faded"
                            } else {
                                "glass featured-card"
                            };
                            rsx! {
                                div {
                                    class: card_cls,
                                    style: "--glow-color:{project.glow_color};",
                                    onclick: toggle_project(project.id, project.glow_color),

                                    if is_expanded {
                                        div { class: "project-content-full",
                                            h3 { "{project.name}" }
                                            for para in &project.detailed_description {
                                                p { "{para}" }
                                            }
                                            div { class: "tags",
                                                for tech in &project.technologies {
                                                    span { "{tech}" }
                                                }
                                            }
                                            div { class: "project-links",
                                                if let Some(url) = project.github_url {
                                                    a {
                                                        href: "{url}",
                                                        target: "_blank",
                                                        rel: "noopener",
                                                        class: "link",
                                                        onclick: |e: Event<MouseData>| e.stop_propagation(),
                                                        "GitHub ↗"
                                                    }
                                                }
                                                if let Some(url) = project.demo_url {
                                                    a {
                                                        href: "{url}",
                                                        target: "_blank",
                                                        rel: "noopener",
                                                        class: "link",
                                                        onclick: |e: Event<MouseData>| e.stop_propagation(),
                                                        "Live Demo ↗"
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        div { class: "project-content-brief",
                                            h3 { "{project.name}" }
                                            p { class: "brief", "{project.brief}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Career ───────────────────────────────────────────────
        section { id: "home-career",
            h2 { class: "section-title", "Career" }
            div { class: "container",
                div { class: "timeline",
                    for entry in &career {
                        div { class: "timeline-item",
                            h3 { "{entry.title}" }
                            p { class: "meta", "{entry.meta}" }
                            if let Some(first) = entry.bullets.first() {
                                p { "{first}" }
                            }
                        }
                    }
                }
            }
        }

        // ── Contact ──────────────────────────────────────────────
        section { id: "home-contact",
            h2 { class: "section-title", "Get in Touch" }
            div { class: "container",
                div { class: "contact-layout",
                    div { class: "glass",
                        h3 { "Let's Build Something" }
                        p { "Have an idea, a project or an opportunity? I'd love to hear about it." }
                        div { class: "contact-methods",
                            div { class: "contact-method",
                                h4 { "Name" }
                                p { "{profile.full_name}" }
                            }
                            div { class: "contact-method",
                                h4 { "Email" }
                                p {
                                    a { href: "mailto:{profile.email}", "{profile.email}" }
                                }
                            }
                            div { class: "contact-method",
                                h4 { "Location" }
                                p { "{profile.location}" }
                            }
                        }
                        div { class: "social-links",
                            h4 { "Follow Me" }
                            div { class: "social-icons",
                                a {
                                    href: "https://github.com/GitanElyon",
                                    target: "_blank",
                                    rel: "noopener me",
                                    "GitHub"
                                }
                                a {
                                    href: "https://www.linkedin.com/in/gitanelyon/",
                                    target: "_blank",
                                    rel: "noopener me",
                                    "LinkedIn"
                                }
                            }
                        }
                    }
                    div { class: "glass",
                        ContactForm {}
                    }
                }
            }
        }
    }
}
