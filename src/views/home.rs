use crate::data::projects::get_projects;
use crate::data::skills::{get_about_languages, get_skill_sections};
use dioxus::prelude::*;

// ── Shared types (used by contact.rs) ────────────────────────────────────

#[derive(Clone, Default, serde::Serialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

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
        _ => "#888888",
    }
}

fn build_skill_groups(skills: &[crate::data::skills::Skill]) -> Vec<Vec<usize>> {
    let n = skills.len();
    if n <= 6 {
        return vec![(0..n).collect()];
    }
    let top = (n + 1) / 2;
    let bottom = n - top;
    let _ = bottom;
    vec![(0..top).collect(), (top..n).collect()]
}

const HEADSHOT: Asset = asset!("/assets/profile.jpg");

// ── Home component ──────────────────────────────────────────────────────

#[component]
pub fn Home() -> Element {
    let sections = get_skill_sections();
    let projects = get_projects();
    let languages = get_about_languages();

    let mut carousel_idx = use_signal(|| 0usize);
    let mut selected_skill = use_signal(|| None::<String>);
    let mut expanded_lang = use_signal(|| None::<String>);
    let mut expanded_project = use_signal(|| None::<String>);

    let mut form_data = use_signal(FormData::default);
    let mut form_status = use_signal(|| FormStatus::Idle);

    let featured: Vec<_> = projects.iter().filter(|p| p.is_featured).collect();

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        form_status.set(FormStatus::Submitting);
        spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .post("https://formspree.io/f/mldnapzn")
                .header("Accept", "application/json")
                .json(&form_data.read().clone())
                .send()
                .await;
            match response {
                Ok(res) if res.status().is_success() => {
                    form_status.set(FormStatus::Success);
                    form_data.set(FormData::default());
                }
                Ok(res) => {
                    let t = res.text().await.unwrap_or_else(|_| "Unknown error".into());
                    form_status.set(FormStatus::Error(format!("Failed: {t}")));
                }
                Err(e) => form_status.set(FormStatus::Error(format!("Network error: {e}"))),
            }
        });
    };

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
                    h2 { "Full Stack Developer & Software Engineer" }
                    p {
                        "I build high‑performance applications with modern tools. Passionate about solving real problems using cutting edge technologies."
                    }
                    div { class: "hero-buttons",
                        Link { to: "/projects", class: "btn btn-primary", "View My Work" }
                        Link { to: "/contact", class: "btn btn-secondary", "Get in Touch" }
                    }
                }
                div { class: "hero-image",
                    img { src: HEADSHOT, alt: "Gitan Elyon" }
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
                            "A passionate software engineer focused on building fast, reliable and beautiful applications. From embedded systems to full‑stack web apps, I love turning ideas into real products."
                        }
                    }
                    // Work card
                    div { class: "glass",
                        h3 { "What I Do" }
                        ul {
                            li { "Full‑stack web applications" }
                            li { "Systems programming & optimization" }
                            li { "Desktop & cross‑platform apps" }
                            li { "Open source contributions" }
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
            div { class: "carousel-wrapper",
                {
                    let section = &sections[carousel_idx()];
                    let groups = build_skill_groups(&section.1);
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

                            // Icon rows
                            for (row_i , row) in groups.iter().enumerate() {
                                div { class: "skills-icon-row",
                                    for & idx in row {
                                        {
                                            let skill = &section.1[idx];
                                            let name = skill.name.to_string();
                                            let color = skill_color(skill.name).to_string();
                                            rsx! {
                                                div { class: if row_i > 0 { "skill-icon-wrap overlap" } else { "skill-icon-wrap" },
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
                                                        img { src: skill.icon, alt: "{skill.name}" }
                                                    }
                                                    span { class: "skill-icon-label", "{skill.name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Bar graph — vertical bars for all tabs (CSS flips to horizontal on mobile)
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
                                            p { "{project.description}" }
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
                    div { class: "timeline-item",
                        h3 { "Full Stack Developer" }
                        p { class: "meta", "NexSys | May 2025 – Present" }
                        p {
                            "Built responsive web applications using Rust, Tauri and Dioxus. Collaborated with cross‑functional teams in an Agile environment."
                        }
                    }
                    div { class: "timeline-item",
                        h3 { "Software Engineer" }
                        p { class: "meta", "Freelancing | 2022 – Present" }
                        p {
                            "Built lightweight applications using JavaScript and Rust. Worked as a solo developer on a wide variety of tasks for clients."
                        }
                    }
                    div { class: "timeline-item",
                        h3 { "Backend Infrastructure Engineer" }
                        p { class: "meta", "Mind Over Machines | May 2024 – Sep 2024" }
                        p {
                            "Optimized digital applications and systems for clients. Implemented systems to accelerate technology development in a fast‑paced environment."
                        }
                    }
                    div { class: "timeline-item",
                        h3 { "Backend Engineer" }
                        p { class: "meta", "Gulp Data | May 2023 – Sep 2023" }
                        p {
                            "Developed and maintained website backend using Python. Improved QA tooling with AI‑powered searching algorithms."
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
                        ul {
                            li { "gitanelyon@gmail.com" }
                            li { "(443)-224-8540" }
                            li { "Baltimore, MD" }
                        }
                    }
                    div { class: "glass",
                        match form_status() {
                            FormStatus::Success => rsx! {
                                div { class: "success-message",
                                    h3 { "Sent!" }
                                    p { "Thanks for reaching out – I'll reply soon." }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| form_status.set(FormStatus::Idle),
                                        "Send Another"
                                    }
                                }
                            },
                            FormStatus::Error(ref msg) => rsx! {
                                div { class: "error-message",
                                    h3 { "Something Went Wrong" }
                                    p { "{msg}" }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| form_status.set(FormStatus::Idle),
                                        "Try Again"
                                    }
                                }
                            },
                            _ => rsx! {
                                form { onsubmit: handle_submit,
                                    div { class: "form-group",
                                        label { "Name" }
                                        input {
                                            r#type: "text",
                                            required: true,
                                            value: "{form_data.read().name}",
                                            oninput: move |e| form_data.write().name = e.value(),
                                        }
                                    }
                                    div { class: "form-group",
                                        label { "Email" }
                                        input {
                                            r#type: "email",
                                            required: true,
                                            value: "{form_data.read().email}",
                                            oninput: move |e| form_data.write().email = e.value(),
                                        }
                                    }
                                    div { class: "form-group",
                                        label { "Message" }
                                        textarea {
                                            required: true,
                                            value: "{form_data.read().message}",
                                            oninput: move |e| form_data.write().message = e.value(),
                                        }
                                    }
                                    button {
                                        class: "btn btn-primary",
                                        r#type: "submit",
                                        disabled: matches!(form_status(), FormStatus::Submitting),
                                        if matches!(form_status(), FormStatus::Submitting) {
                                            "Sending…"
                                        } else {
                                            "Send Message"
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
