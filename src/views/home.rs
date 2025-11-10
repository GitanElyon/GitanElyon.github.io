use crate::Route;
use dioxus::prelude::*;
use serde::Serialize;
use std::rc::Rc;

const PROFILE_IMAGE: Asset = asset!("/assets/profile.jpg");

// Home page section stylesheets
const HOME_ABOUT_CSS: Asset = asset!("/assets/styling/home-about.css");
const HOME_HERO_CSS: Asset = asset!("/assets/styling/home-hero.css");
const HOME_SKILLS_CSS: Asset = asset!("/assets/styling/home-skills.css");
const HOME_PROJECTS_CSS: Asset = asset!("/assets/styling/home-projects.css");
const HOME_CAREER_CSS: Asset = asset!("/assets/styling/home-career.css");
const HOME_CONTACT_CSS: Asset = asset!("/assets/styling/home-contact.css");

// Define icon assets
const RUST_ICON: Asset = asset!("/icons/langs/rust.svg");
const GO_ICON: Asset = asset!("/icons/langs/go.svg");
const PYTHON_ICON: Asset = asset!("/icons/langs/python.svg");
const JS_ICON: Asset = asset!("/icons/langs/js.svg");
const TS_ICON: Asset = asset!("/icons/langs/ts.svg");
const JAVA_ICON: Asset = asset!("/icons/langs/java.svg");
const HTML_ICON: Asset = asset!("/icons/langs/html-5.svg");
const CSS_ICON: Asset = asset!("/icons/langs/css-3.svg");
const C_ICON: Asset = asset!("/icons/langs/c.svg");
const CPP_ICON: Asset = asset!("/icons/langs/cpp.svg");
const CSHARP_ICON: Asset = asset!("/icons/langs/csharp.svg");
const LUA_ICON: Asset = asset!("/icons/langs/lua.svg");

const TAURI_ICON: Asset = asset!("/icons/frameworks/tauri.svg");
const BUN_ICON: Asset = asset!("/icons/frameworks/bun.svg");
const DIOXUS_ICON: Asset = asset!("/icons/frameworks/dioxus.png");
const NODE_ICON: Asset = asset!("/icons/frameworks/node-js.svg");
const SVELTE_ICON: Asset = asset!("/icons/frameworks/svelte.svg");
const VUE_ICON: Asset = asset!("/icons/frameworks/vue.svg");
const REACT_ICON: Asset = asset!("/icons/frameworks/react.svg");
const ELECTRON_ICON: Asset = asset!("/icons/frameworks/electron.svg");

const LINUX_ICON: Asset = asset!("/icons/tools/linux.svg");
const GIT_ICON: Asset = asset!("/icons/tools/git.svg");
const NIX_ICON: Asset = asset!("/icons/tools/nix.svg");
const DOCKER_ICON: Asset = asset!("/icons/tools/docker.svg");
const ADOBE_ICON: Asset = asset!("/icons/tools/adobe.svg");
const FIGMA_ICON: Asset = asset!("/icons/tools/figma.svg");

const POSTGRES_ICON: Asset = asset!("/icons/data/postgresql.svg");
const SOLR_ICON: Asset = asset!("/icons/data/apachesolr.svg");
const SURREALDB_ICON: Asset = asset!("/icons/data/surrealdb.svg");

// Skill with icon path, name, hours, and projects
#[derive(Clone)]
struct Skill {
    name: &'static str,
    icon: Asset,
    hours: u32,
    projects: Vec<&'static str>,
}

// Small data model for About -> Main Technologies
#[derive(Clone)]
struct LanguageAbout {
    name: &'static str,
    brief: &'static str,
    long: &'static str,
    years: &'static str,
    tools: Vec<&'static str>,
    color: &'static str, // accent color for hover glow
}

#[derive(Clone, PartialEq)]
enum FormStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[derive(Clone, Default, Serialize)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn Home() -> Element {
    // Skills data grouped into 4 sections with icons and primary colors
    let skill_sections: Rc<Vec<(&'static str, Vec<Skill>)>> = Rc::new(vec![
        ("Languages", vec![
            Skill { name: "Rust", icon: RUST_ICON, hours: 400, projects: vec!["Minos", "Portfolio", "Blackjack AI", "Suilend Liquidator"] },
            Skill { name: "Go", icon: GO_ICON, hours: 50, projects: vec!["Backend APIs", "Microservices"] },
            Skill { name: "Python", icon: PYTHON_ICON, hours: 100, projects: vec!["ML Models", "Data Analysis", "Automation"] },
            Skill { name: "JavaScript", icon: JS_ICON, hours: 200, projects: vec!["Web Apps", "Node Services"] },
            Skill { name: "TypeScript", icon: TS_ICON, hours: 150, projects: vec!["Full Stack Apps", "Type-safe APIs"] },
            Skill { name: "Java", icon: JAVA_ICON, hours: 80, projects: vec!["Enterprise Apps", "Android"] },
            Skill { name: "HTML", icon: HTML_ICON, hours: 120, projects: vec!["Web Development"] },
            Skill { name: "CSS", icon: CSS_ICON, hours: 100, projects: vec!["UI/UX Design"] },
            Skill { name: "C", icon: C_ICON, hours: 50, projects: vec!["Systems Programming"] },
            Skill { name: "C++", icon: CPP_ICON, hours: 20, projects: vec!["Game Development"] },
            Skill { name: "C#", icon: CSHARP_ICON, hours: 60, projects: vec![".NET Apps"] },
            Skill { name: "Lua", icon: LUA_ICON, hours: 30, projects: vec!["Scripting", "Neovim Config"] },
        ]),
        ("Frameworks", vec![
            Skill { name: "Tauri", icon: TAURI_ICON, hours: 200, projects: vec!["Desktop Apps"] },
            Skill { name: "Bun.js", icon: BUN_ICON, hours: 120, projects: vec!["Fast Runtimes"] },
            Skill { name: "Dioxus", icon: DIOXUS_ICON, hours: 150, projects: vec!["This Portfolio!"] },
            Skill { name: "Node.js", icon: NODE_ICON, hours: 40, projects: vec!["Backend Services"] },
            Skill { name: "Svelte", icon: SVELTE_ICON, hours: 160, projects: vec!["Interactive UIs"] },
            Skill { name: "Vue", icon: VUE_ICON, hours: 80, projects: vec!["Papyr"] },
            Skill { name: "React", icon: REACT_ICON, hours: 20, projects: vec!["Web Apps"] },
            Skill { name: "Electron", icon: ELECTRON_ICON, hours: 40, projects: vec!["Cross-Platform Apps"] },
        ]),
        ("Tools", vec![
            Skill { name: "Linux", icon: LINUX_ICON, hours: 1500, projects: vec!["Daily Driver", "Server Management"] },
            Skill { name: "Git", icon: GIT_ICON, hours: 800, projects: vec!["Version Control", "Collaboration"] },
            Skill { name: "Nix", icon: NIX_ICON, hours: 700, projects: vec!["Reproducible Builds"] },
            Skill { name: "Docker", icon: DOCKER_ICON, hours: 100, projects: vec!["Containerization"] },
            Skill { name: "Adobe", icon: ADOBE_ICON, hours: 200, projects: vec!["Design Work"] },
            Skill { name: "Figma", icon: FIGMA_ICON, hours: 150, projects: vec!["UI Design"] },
        ]),
        ("Databases", vec![
            Skill { name: "PostgreSQL", icon: POSTGRES_ICON, hours: 50, projects: vec!["Relational DBs"] },
            Skill { name: "Solr", icon: SOLR_ICON, hours: 30, projects: vec!["Search Engines"] },
            Skill { name: "SurrealDB", icon: SURREALDB_ICON, hours: 20, projects: vec!["Modern DBs"] },
        ]),
    ]);

    // Data for About -> Main Technologies vertical list
    let main_langs: Vec<LanguageAbout> = vec![
        LanguageAbout {
            name: "Rust",
            brief: "Memory-safe and performant",
            long: "Rust is my go-to language for building memory-safe, high-performance applications. Rust often becomes the core of my projects thanks to its reliability and speed.",
            years: "~1.5 years",
            tools: vec!["Tauri", "Dioxus", "Suilend"],
            color: "#DEA584",
        },
        LanguageAbout {
            name: "Go",
            brief: "Simple, fast and reliable",
            long: "Go is my choice for building scalable backend services with simplicity and efficiency.",
            years: "~1 year",
            tools: vec!["Microservices", "REST", "gRPC"],
            color: "#00ADD8",
        },
        LanguageAbout {
            name: "Python",
            brief: "Versatile and powerful",
            long: "Python is my tool for prototyping, scripting, and automating anything I need done quickly.",
            years: "~4 years",
            tools: vec!["Pandas", "FastAPI", "NumPy"],
            color: "#3776AB",
        },
        LanguageAbout {
            name: "JavaScript",
            brief: "Dynamic and flexible",
            long: "JavaScript isn't my favorite stack to write in, but if I can dream it, JS can make it happen.",
            years: "~2.5 years",
            tools: vec!["Node", "Vue", "Svelte"],
            color: "#F7DF1E",
        },
    ];

    // State for carousel index & expanded project
    let mut current_section = use_signal(|| 0usize);
    let mut expanded_project = use_signal(|| None::<usize>);
    let mut form_data = use_signal(FormData::default);
    let mut form_status = use_signal(|| FormStatus::Idle);
    let mut slide_direction = use_signal(|| "");
    let mut selected_skill = use_signal(|| None::<usize>);
    // About -> Main Technologies state
    let mut selected_lang_about = use_signal(|| None::<usize>);

    // Helper to cycle index with animation
    let prev_section = {
        let skill_sections = skill_sections.clone();
        move |_| {
            slide_direction.set("slide-left");
            let cur = current_section();
            current_section.set(if cur == 0 { skill_sections.len()-1 } else { cur - 1 });
        }
    };
    let next_section = {
        let skill_sections = skill_sections.clone();
        move |_| {
            slide_direction.set("slide-right");
            let cur = current_section();
            current_section.set((cur + 1) % skill_sections.len());
        }
    };

    // Toggle expansion for a given project id
    let toggle_project = |id: usize| move |_| {
        if expanded_project() == Some(id) { expanded_project.set(None); } else { expanded_project.set(Some(id)); }
    };

    // Form submission handler
    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        form_status.set(FormStatus::Submitting);

        spawn(async move {
            let formspree_url = "https://formspree.io/f/mldnapzn";

            let client = reqwest::Client::new();
            let response = client
                .post(formspree_url)
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
                    let error_text = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "An unknown error occurred.".to_string());
                    form_status.set(FormStatus::Error(format!("Failed to send message: {}", error_text)));
                }
                Err(err) => {
                    form_status.set(FormStatus::Error(format!("Network error: {}", err)));
                }
            }
        });
    };

    rsx! {
        // Load per-section styles for the home page
        document::Link { rel: "stylesheet", href: HOME_ABOUT_CSS }
        document::Link { rel: "stylesheet", href: HOME_HERO_CSS }
        document::Link { rel: "stylesheet", href: HOME_SKILLS_CSS }
        document::Link { rel: "stylesheet", href: HOME_PROJECTS_CSS }
        document::Link { rel: "stylesheet", href: HOME_CAREER_CSS }
        document::Link { rel: "stylesheet", href: HOME_CONTACT_CSS }

        // HERO --------------------------------------------------------------
        section {
            id: "hero",
            div {
                class: "hero-content",
                div {
                    class: "hero-text",
                    h1 { "Gitan Elyon Mandell-Balogh" }
                    h2 { "Software Engineer & Full-Stack Developer" }
                    p {
                        "Passionate about creating innovative solutions with modern web technologies. 
                        Specializing in Rust, TypeScript, and building scalable applications."
                    }
                    div {
                        class: "hero-buttons",
                        Link {
                            to: Route::Projects {},
                            class: "btn btn-primary",
                            "View My Work"
                        }
                        Link {
                            to: Route::Contact {},
                            class: "btn btn-secondary",
                            "Get In Touch"
                        }
                    }
                }
                div {
                    class: "hero-image",
                    img { src: PROFILE_IMAGE, alt: "Gitan Elyon" }
                }
            }
        }
        // ABOUT --------------------------------------------------------------
        section { id: "about-section", class: "home-section about-center home-about",
            h2 { class: "section-title", "About Me" }
            div { class: "glass-bw about-card inner",
                div { class: "about-grid",
                    div { class: "gen-info",
                        h3 { "Who Am I?" }
                        p { "I'm a passionate software engineer with a strong background in full‑stack and systems development. I build efficient, user‑centric applications and explore ways to push performance and reliability." }
                        p { "My journey started with solving everyday problems. Since then I've worked across several languages and platforms refining a pragmatic mindset toward shipping maintainable, high‑quality software." }
                    }
                    div { class: "current-work",
                        h3 { "Currently Working On" }
                        h4 { "Papyr" }
                        p { "A lightweight, real‑time markdown editor component designed to seamlessly integrate into web apps with a focus on speed and customization." }
                        p { "Demo coming soon!" }
                        a { href: "https://github.com/gitanelyon/papyr", target: "_blank", class: "link", "GitHub ↗" }
                        div { class: "tags",
                            span { "TypeScript" }
                            span { "HTML" }
                            span { "CSS" }
                            span { "Markdown" }
                        }
                    }
                    div { class: "about-languages",
                        h3 { "Main Technologies" }
                        // Vertical list of main languages - expand on click only
                        div { class: "lang-list",
                            if let Some(sel_idx) = selected_lang_about() {
                                {
                                    let lang = &main_langs[sel_idx];
                                    let skill_opt = skill_sections[0].1.iter().find(|s| s.name == lang.name);
                                    let hours = skill_opt.map(|s| s.hours).unwrap_or(0);
                                    let projects: Vec<&'static str> = skill_opt.map(|s| s.projects.clone()).unwrap_or_default();
                                    let projects_count = projects.len();

                                    rsx! {
                                        div { class: "lang-expanded", style: "--glow: {lang.color};",
                                            onclick: move |_| selected_lang_about.set(None),
                                            h4 { "{lang.name} | {lang.brief}" }
                                            p { "{lang.long}" }
                                            p { class: "stats", "Stats: {lang.years}, +{hours} hours, {projects_count} projects." }
                                            if !lang.tools.is_empty() {
                                                p { class: "tools",
                                                    b { "Tools: " }
                                                    for (i, t) in lang.tools.iter().enumerate() {
                                                        span { "{t}" }
                                                        if i < lang.tools.len()-1 { span { ", " } }
                                                    }
                                                }
                                            }
                                            if !projects.is_empty() {
                                                p { class: "projects",
                                                    b { "Projects: " }
                                                    for (i, prj) in projects.iter().enumerate() {
                                                        span { "{prj}" }
                                                        if i < projects.len()-1 { span { ", " } }
                                                    }
                                                }
                                            }
                                            p { class: "hint", "Click to collapse" }
                                        }
                                    }
                                }
                            } else {
                                for (idx, lang) in main_langs.iter().enumerate() {
                                    div { class: "lang-row", style: "--glow: {lang.color};",
                                        onclick: move |_| selected_lang_about.set(Some(idx)),
                                        span { class: "lang-name", "{lang.name}" }
                                        span { class: "lang-sep", " | " }
                                        span { class: "lang-brief", "{lang.brief}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // SKILLS -------------------------------------------------------------
        section { id: "skills-section", class: "home-section skills-pane",
            h2 { class: "section-title", "Skills" }
            div { class: "carousel-wrapper",
                button { class: "nav-arrow left", onclick: prev_section, "←" }
                div { class: "glass-bw skills-glass {slide_direction()}",
                    h3 { class: "category-title", "{skill_sections[current_section()].0}" }
                    
                    // Icon row with overlapping similar languages
                    div { class: "skills-icon-row",
                        for (idx, skill) in skill_sections[current_section()].1.iter().enumerate() {
                            {
                                let is_overlap = match skill.name {
                                    "CSS" | "TypeScript" | "C++" => true,
                                    _ => false,
                                };
                                let is_selected = selected_skill() == Some(idx);
                                
                                rsx! {
                                    div { 
                                        class: if is_overlap { "skill-icon-wrap overlap" } else { "skill-icon-wrap" },
                                        div {
                                            class: if is_selected { "skill-icon selected" } else { "skill-icon" },
                                            "data-skill": "{skill.name}",
                                            onmouseenter: move |_| selected_skill.set(Some(idx)),
                                            onmouseleave: move |_| selected_skill.set(None),
                                            img { src: skill.icon, alt: "{skill.name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    // Stats display below
                    div { class: "skills-stats",
                        if let Some(idx) = selected_skill() {
                            if let Some(skill) = skill_sections[current_section()].1.get(idx) {
                                div { class: "stats-content",
                                    h3 { "{skill.name}" }
                                    p { class: "hours", "{skill.hours}+ hours of experience" }
                                    div { class: "projects-showcase",
                                        for project in skill.projects.iter() {
                                            span { class: "project-pill", "{project}" }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Bar graph overview
                            div { class: "stats-overview",
                                h4 { "Experience Overview" }
                                div { class: "bar-graph",
                                    for skill in skill_sections[current_section()].1.iter() {
                                        {
                                            let max_hours = skill_sections[current_section()].1.iter()
                                                .map(|s| s.hours)
                                                .max()
                                                .unwrap_or(1);
                                            let percentage = (skill.hours as f64 / max_hours as f64 * 100.0) as u32;
                                            
                                            rsx! {
                                                div { class: "bar-item",
                                                    div { class: "bar-label", "{skill.name}" }
                                                    div { class: "bar-wrapper",
                                                        div { 
                                                            class: "bar-fill",
                                                            "data-skill": "{skill.name}",
                                                            style: "width: {percentage}%",
                                                        }
                                                        span { class: "bar-hours", "{skill.hours}h" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    div { class: "dots",
                        for i in 0..skill_sections.len() {
                            button {
                                class: if i == current_section() { "dot active" } else { "dot" },
                                onclick: move |_| {
                                    current_section.set(i);
                                    slide_direction.set("");
                                    selected_skill.set(None);
                                },
                            }
                        }
                    }
                }
                button { class: "nav-arrow right", onclick: next_section, "→" }
            }
        }

        // PROJECTS -----------------------------------------------------------
        section { id: "projects-section", class: "home-section projects-flex",
            h2 { "Featured Projects" }
            div { class: "projects-row",
                if expanded_project().is_none() {
                    // Collapsed: show 3 side-by-side
                    div { class: "project-card-bw", onclick: toggle_project(0),
                        h3 { "Minos" }
                        p { class: "brief", "Real‑time AI Tetris player" }
                    }
                    div { class: "project-card-bw", onclick: toggle_project(1),
                        h3 { "Papyr" }
                        p { class: "brief", "Real‑time markdown editor" }
                    }
                    div { class: "project-card-bw", onclick: toggle_project(2),
                        h3 { "Rust Liquidator" }
                        p { class: "brief", "High‑performance trading bot" }
                    }
                } else if expanded_project() == Some(0) {
                    // Expanded: show only Minos
                    div { class: "project-card-bw expanded", onclick: toggle_project(0),
                        h3 { "Minos" }
                        p { class: "detail", "AI that plays Tetris in real time combining Python screen parsing with Rust computation for optimal moves." }
                        div { class: "tags", span { "Rust" } span { "Python" } span { "AI" } span { "ML" } }
                        a { href: "https://github.com/GitanElyon/minos", target: "_blank", class: "link", "GitHub ↗" }
                    }
                } else if expanded_project() == Some(1) {
                    // Expanded: show only Papyr
                    div { class: "project-card-bw expanded", onclick: toggle_project(1),
                        h3 { "Papyr" }
                        p { class: "detail", "A real‑time markdown editor focused on speed, simplicity and staying organized across notes." }
                        div { class: "tags", span { "Vue" } span { "JavaScript" } span { "Markdown" } span { "CSS" } }
                        a { href: "https://github.com/GitanElyon/papyr", target: "_blank", class: "link", "GitHub ↗" }
                    }
                } else {
                    // Expanded: show only Rust Liquidator
                    div { class: "project-card-bw expanded", onclick: toggle_project(2),
                        h3 { "Rust Liquidator" }
                        p { class: "detail", "High‑performance algorithmic trading bot leveraging async Rust, orderbook streaming & rapid decision logic." }
                        div { class: "tags", span { "Rust" } span { "Async" } span { "WebSockets" } span { "Trading" } }
                        p { class: "note", "Private Repository" }
                    }
                }
            }
        }

        // CAREER -------------------------------------------------------------
        section { id: "career-section", class: "home-section career-pane",
            h2 { "Career" }
            div { class: "timeline glass-bw",
                div { class: "job", h3 { "Full Stack Developer" } p { class: "meta", "NexSys • 2025 - Present" } p { "Rust + Tauri + Dioxus apps, CI/CD, agile collaboration." } }
                div { class: "job", h3 { "Software Engineer" } p { class: "meta", "Freelance • 2022 - Present" } p { "Delivered solo full‑stack & tooling projects solving practical problems." } }
                div { class: "job", h3 { "Backend Infrastructure Engineer" } p { class: "meta", "Mind Over Machines • 2024" } p { "Optimized client systems & accelerated development pathways." } }
                div { class: "job", h3 { "Backend Engineer" } p { class: "meta", "Gulp Data • 2023" } p { "Python backend maintenance, AI search improvements." } }
            }
        }

        // CONTACT ------------------------------------------------------------
        section { id: "contact-section", class: "home-section contact-pane",
            div { class: "contact-container",
                h2 { "Get In Touch" }
                div { class: "contact-layout",
                    // Left: Contact Info
                    div { class: "glass-bw contact-info",
                        h3 { "Let's Connect" }
                        p {
                            "I'm always interested in hearing about new opportunities,
                            exciting projects, or just having a chat about technology."
                        }

                        div { class: "contact-methods",
                            div { class: "contact-method",
                                h4 { "Email" }
                                p { "gitanelyon@gmail.com" }
                            }
                            div { class: "contact-method",
                                h4 { "Phone" }
                                p { "(443)-224-8540" }
                            }
                            div { class: "contact-method",
                                h4 { "Location" }
                                p { "Baltimore, Maryland" }
                            }
                        }

                        div { class: "social-links",
                            h4 { "Follow Me" }
                            div { class: "social-icons",
                                a { href: "https://github.com/GitanElyon", target: "_blank", "GitHub ↗"}
                                a { href: "https://linkedin.com/in/gitaneylon", target: "_blank", "LinkedIn" }
                            }
                        }

                        div { class: "resume-btn-wrapper",
                            Link { to: Route::Resume {}, class: "btn-resume", "View Resume" }
                        }
                    }

                    // Right: Contact Form
                    div { class: "glass-bw contact-form-wrapper",
                        match form_status() {
                            FormStatus::Success => rsx! {
                                div { class: "success-message",
                                    h3 { "Message Sent!" }
                                    p { "Thanks for reaching out. I'll get back to you soon." }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| form_status.set(FormStatus::Idle),
                                        "Send Another"
                                    }
                                }
                            },
                            FormStatus::Error(error_msg) => rsx! {
                                div { class: "error-message",
                                    h3 { "Something Went Wrong" }
                                    p { "{error_msg}" }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| form_status.set(FormStatus::Idle),
                                        "Try Again"
                                    }
                                }
                            },
                            _ => rsx! {
                                form {
                                    class: "contact-form",
                                    onsubmit: handle_submit,

                                    div { class: "form-group",
                                        label { r#for: "name", "Name" }
                                        input {
                                            r#type: "text",
                                            id: "name",
                                            required: true,
                                            disabled: form_status() == FormStatus::Submitting,
                                            value: "{form_data.read().name}",
                                            oninput: move |e| form_data.write().name = e.value(),
                                        }
                                    }

                                    div { class: "form-group",
                                        label { r#for: "email", "Email" }
                                        input {
                                            r#type: "email",
                                            id: "email",
                                            required: true,
                                            disabled: form_status() == FormStatus::Submitting,
                                            value: "{form_data.read().email}",
                                            oninput: move |e| form_data.write().email = e.value(),
                                        }
                                    }

                                    div { class: "form-group",
                                        label { r#for: "message", "Message" }
                                        textarea {
                                            id: "message",
                                            rows: "5",
                                            required: true,
                                            disabled: form_status() == FormStatus::Submitting,
                                            value: "{form_data.read().message}",
                                            oninput: move |e| form_data.write().message = e.value(),
                                        }
                                    }

                                    button {
                                        r#type: "submit",
                                        class: "btn btn-primary",
                                        disabled: form_status() == FormStatus::Submitting,
                                        if form_status() == FormStatus::Submitting {
                                            "Sending..."
                                        } else {
                                            "Send Message"
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
}
