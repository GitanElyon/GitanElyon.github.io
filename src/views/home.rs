use crate::Route;
use crate::data::projects::get_projects;
use crate::data::skills::{Skill, LanguageAbout, get_skill_sections, get_about_languages};
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

fn skill_color(name: &str) -> &'static str {
    match name {
        "Rust" => "#DEA584",
        "Go" => "#00ADD8",
        "Python" => "#3776AB",
        "JavaScript" => "#F7DF1E",
        "TypeScript" => "#3178C6",
        "Java" => "#007396",
        "HTML" => "#E34F26",
        "CSS" => "#1572B6",
        "C" => "#5C6BC0",
        "C++" => "#00599C",
        "C#" => "#512BD4",
        "Lua" => "#2C2D72",
        "Tauri" => "#0EA5E9",
        "Bun.js" => "#F2A550",
        "Dioxus" => "#3E63DD",
        "Node.js" => "#3C873A",
        "Svelte" => "#FF3E00",
        "Vue" => "#41B883",
        "React" => "#61DAFB",
        "Electron" => "#47848F",
        "Linux" => "#FCC624",
        "Git" => "#F05033",
        "Nix" => "#5277C3",
        "Docker" => "#2496ED",
        "Adobe" => "#FF0000",
        "Figma" => "#F24E1E",
        "PostgreSQL" => "#316192",
        "Solr" => "#D9411E",
        "SurrealDB" => "#FF006E",
        _ => "#9CA3AF",
    }
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
    let skill_sections: Rc<Vec<(&'static str, Vec<Skill>)>> = Rc::new(get_skill_sections());

    // Data for About -> Main Technologies vertical list
    let main_langs: Vec<LanguageAbout> = get_about_languages();

    // Data for Projects
    let projects = use_memo(|| get_projects());

    // State for carousel index & expanded project
    let mut current_section = use_signal(|| 0usize);
    let mut expanded_project = use_signal(|| None::<String>);
    let mut form_data = use_signal(FormData::default);
    let mut form_status = use_signal(|| FormStatus::Idle);
    let mut slide_direction = use_signal(|| "");
    let mut selected_skill = use_signal(|| None::<usize>);
    let mut hovered_skill = use_signal(|| None::<usize>);
    // About -> Main Technologies state
    let mut selected_lang_about = use_signal(|| None::<usize>);

    // Helper to cycle index with animation
    let prev_section = {
        let skill_sections = skill_sections.clone();
        move |_| {
            slide_direction.set("slide-left");
            let cur = current_section();
            current_section.set(if cur == 0 {
                skill_sections.len() - 1
            } else {
                cur - 1
            });
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
    let toggle_project = |id: &'static str| {
        let id_string = id.to_string();
        move |_| {
            if expanded_project() == Some(id_string.clone()) {
                expanded_project.set(None);
            } else {
                expanded_project.set(Some(id_string.clone()));
            }
        }
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
                    form_status.set(FormStatus::Error(format!(
                        "Failed to send message: {}",
                        error_text
                    )));
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
        section { id: "hero",
            div { class: "hero-content",
                div { class: "hero-text",
                    h1 { class: "hero-name",
                        span { class: "name-line", "Gitan Elyon" }
                        span { class: "name-line", "Mandell-Balogh" }
                    }
                    h2 { "Software Engineer & Full-Stack Developer" }
                    p {
                        "Passionate about creating innovative solutions with modern web technologies.
                        Specializing in Rust and JavaScript I work to build modern applications."
                    }
                    div { class: "hero-buttons",
                        Link { to: Route::Projects {}, class: "btn btn-primary", "View My Work" }
                        Link { to: Route::Contact {}, class: "btn btn-secondary", "Get In Touch" }
                    }
                }
                div { class: "hero-image",
                    img { src: PROFILE_IMAGE, alt: "Gitan Elyon" }
                }
            }
        }
        // ABOUT --------------------------------------------------------------
        section {
            id: "about-section",
            class: "home-section about-center home-about",
            h2 { class: "section-title", "About Me" }
            div { class: "inner",
                div { class: "about-grid",
                    div { class: "glass-bw gen-info",
                        h3 { "Who Am I?" }
                        p {
                            "I'm a passionate software engineer with a strong background in full‑stack and systems development. I build efficient, user‑centric applications and explore ways to push performance and reliability."
                        }
                        p {
                            "My journey started with solving everyday problems. Since then I've worked across several languages and platforms refining a pragmatic mindset toward shipping maintainable, high‑quality software."
                        }
                    }
                    div { class: "glass-bw current-work",
                        h3 { "Currently Working On" }
                        h4 { "Papyr" }
                        p {
                            "A lightweight, real‑time markdown editor component designed to seamlessly integrate into web apps with a focus on speed and customization."
                        }
                        p { "Demo coming soon!" }
                        a {
                            href: "https://github.com/gitanelyon/papyr",
                            target: "_blank",
                            class: "link",
                            "GitHub ↗"
                        }
                        div { class: "tags",
                            span { "TypeScript" }
                            span { "HTML" }
                            span { "CSS" }
                            span { "Markdown" }
                        }
                    }
                    div { class: "glass-bw about-languages",
                        h3 { "Main Technologies" }
                        p { class: "expand-note", "Tap any technology to expand" }
                        // Vertical list of main languages - expand on click only
                        div { class: "lang-list",
                            if let Some(sel_idx) = selected_lang_about() {
                                {
                                    let lang = &main_langs[sel_idx];
                                    let skill_opt = skill_sections[0].1.iter().find(|s| s.name == lang.name);
                                    let hours = skill_opt.map(|s| s.hours).unwrap_or(0);
                                    let projects: Vec<&'static str> = skill_opt
                                        .map(|s| s.projects.clone())
                                        .unwrap_or_default();
                                    let projects_count = projects.len();
                                    rsx! {
                                        div {
                                            class: "lang-expanded",
                                            style: "--glow: {lang.color};",
                                            onclick: move |_| selected_lang_about.set(None),
                                            h4 { "{lang.name} | {lang.brief}" }
                                            p { "{lang.long}" }
                                            p { class: "stats", "Stats: {lang.years}, +{hours} hours, {projects_count} projects." }
                                            if !lang.tools.is_empty() {
                                                p { class: "tools",
                                                    b { "Tools: " }
                                                    for (i , t) in lang.tools.iter().enumerate() {
                                                        span { "{t}" }
                                                        if i < lang.tools.len() - 1 {
                                                            span { ", " }
                                                        }
                                                    }
                                                }
                                            }
                                            if !projects.is_empty() {
                                                p { class: "projects",
                                                    b { "Projects: " }
                                                    for (i , prj) in projects.iter().enumerate() {
                                                        span { "{prj}" }
                                                        if i < projects.len() - 1 {
                                                            span { ", " }
                                                        }
                                                    }
                                                }
                                            }
                                            p { class: "hint", "Click to collapse" }
                                        }
                                    }
                                }
                            } else {
                                for (idx , lang) in main_langs.iter().enumerate() {
                                    div {
                                        class: "lang-row",
                                        style: "--glow: {lang.color};",
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
            // Tabs for skill sections
            div { class: "skills-tabs",
                for (i , (title , _)) in skill_sections.iter().enumerate() {
                    {
                        let is_active = i == current_section();
                        rsx! {
                            button {
                                class: if is_active { "tab active" } else { "tab" },
                                onclick: move |_| {
                                    let cur = current_section();
                                    if i != cur {
                                        if i > cur {
                                            slide_direction.set("slide-right");
                                        } else {
                                            slide_direction.set("slide-left");
                                        }
                                        current_section.set(i);
                                        selected_skill.set(None);
                                    }
                                },
                                "{title}"
                            }
                        }
                    }
                }
            }
            div { class: "carousel-wrapper",
                button { class: "nav-arrow left", onclick: prev_section, "←" }
                div { class: "glass-bw skills-glass {slide_direction()}",
                    h3 { class: "category-title", "{skill_sections[current_section()].0}" }

                    div { class: "skills-inner",
                        // Icon row with overlapping similar languages
                        div { class: "skills-icon-row",
                            for (idx , skill) in skill_sections[current_section()].1.iter().enumerate() {
                                {
                                    let is_overlap = match skill.name {
                                        "CSS" | "TypeScript" | "C++" => true,
                                        _ => false,
                                    };
                                    let is_selected = selected_skill() == Some(idx);

                                    rsx! {
                                        div { class: if is_overlap { "skill-icon-wrap overlap" } else { "skill-icon-wrap" },
                                            div {
                                                class: if is_selected { "skill-icon selected" } else { "skill-icon" },
                                                "data-skill": "{skill.name}",
                                                onmouseenter: move |_| hovered_skill.set(Some(idx)),
                                                onmouseleave: move |_| hovered_skill.set(None),
                                                onclick: move |_| {
                                                    if selected_skill() == Some(idx) {
                                                        selected_skill.set(None);
                                                    } else {
                                                        selected_skill.set(Some(idx));
                                                    }
                                                },
                                                img { src: skill.icon, alt: "{skill.name}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Mobile: stacked list with inline progress charts (grouped like desktop)
                        {
                            let section_idx = current_section();
                            let skills = &skill_sections[section_idx].1;

                            // Build groups (same logic as desktop bar graph)
                            let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
                            if section_idx == 0 {
                                let mut i = 0;
                                while i < skills.len() {
                                    let skill = &skills[i];
                                    match skill.name {
                                        "JavaScript" => {
                                            if let Some(next_skill) = skills.get(i + 1) {
                                                if next_skill.name == "TypeScript" {
                                                    groups.push(("JS / TS".to_string(), vec![i, i + 1]));
                                                    i += 2;
                                                    continue;
                                                }
                                            }
                                            groups.push((skill.name.to_string(), vec![i]));
                                            i += 1;
                                        }
                                        "HTML" => {
                                            if let Some(next_skill) = skills.get(i + 1) {
                                                if next_skill.name == "CSS" {
                                                    groups.push(("HTML / CSS".to_string(), vec![i, i + 1]));
                                                    i += 2;
                                                    continue;
                                                }
                                            }
                                            groups.push((skill.name.to_string(), vec![i]));
                                            i += 1;
                                        }
                                        "C" => {
                                            if let Some(next_skill) = skills.get(i + 1) {
                                                if next_skill.name == "C++" {
                                                    groups.push(("C / C++".to_string(), vec![i, i + 1]));
                                                    i += 2;
                                                    continue;
                                                }
                                            }
                                            groups.push((skill.name.to_string(), vec![i]));
                                            i += 1;
                                        }
                                        _ => {
                                            groups.push((skill.name.to_string(), vec![i]));
                                            i += 1;
                                        }
                                    }
                                }
                            } else {
                                for (idx, skill) in skills.iter().enumerate() {
                                    groups.push((skill.name.to_string(), vec![idx]));
                                }
                            }

                            // Calculate max hours from grouped totals
                            let max_hours = groups
                                .iter()
                                .map(|(_, indices)| {
                                    indices.iter().map(|&idx| skills[idx].hours).sum::<u32>()
                                })
                                .max()
                                .unwrap_or(1);
                            rsx! { // Collect all projects from all skills in the group
                                div { class: "skills-mobile-list",
                                    for (group_idx , (label , skill_indices)) in groups.iter().enumerate() {

                                        {
                                            // Combined hours for the group
                                            let combined_hours: u32 = skill_indices
                                                .iter() // Use first skill's color for the group  Use first skill's color for the group
                                                .map(|&idx| skills[idx].hours)
                                                .sum(); // Collect all projects from all skills in the group  Collect all projects from all skills in the group
                                            let percentage = ((combined_hours as f64 / max_hours as f64) * 100.0).round()
                                                as u32;
                                            let primary_skill = &skills[skill_indices[0]];
                                            let skill_glow = skill_color(primary_skill.name);
                                            let is_selected = selected_skill() == Some(group_idx);
                                            let all_projects: Vec<&'static str> = skill_indices
                                                .iter()
                                                .flat_map(|&idx| skills[idx].projects.clone())
                                                .collect();
                                            let project_count = all_projects.len();
                                            rsx! {
                                                div {
                                                    class: if is_selected { "mobile-skill-row selected" } else { "mobile-skill-row" },
                                                    onclick: move |_| {
                                                        if selected_skill() == Some(group_idx) {
                                                            selected_skill.set(None);
                                                        } else {
                                                            selected_skill.set(Some(group_idx));
                                                        }
                                                    },
                                                    div { class: "mobile-skill-meta",
                                                        div { class: "mobile-skill-icons",
                                                            for & idx in skill_indices.iter() {
                                                                img { src: skills[idx].icon, alt: "{skills[idx].name}" }
                                                            }
                                                        }
                                                        span { class: "mobile-skill-name", "{label}" }
                                                    }
                                                    div { class: "mobile-skill-chart",
                                                        div { class: "mobile-skill-track",
                                                            div {
                                                                class: "mobile-skill-fill",
                                                                style: "width: {percentage}%; --skill-color: {skill_glow};",
                                                            }
                                                        }
                                                        span { class: "mobile-skill-hours", "{combined_hours}h" }
                                                    }

                                                    if is_selected {
                                                        div { class: "mobile-skill-projects",
                                                            span { class: "mobile-skill-project-header", "{project_count} projects:" }
                                                            div { class: "mobile-skill-pills",
                                                                for project in all_projects.iter() {
                                                                    span { class: "mobile-skill-pill", "{project}" }
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
                                    {
                                        let section_idx = current_section();
                                        let hovered_idx = hovered_skill();
                                        let skills = &skill_sections[section_idx].1;

                                        let mut groups: Vec<(String, Vec<usize>, u32)> = Vec::new();

                                        if section_idx == 0 {
                                            let mut i = 0;
                                            while i < skills.len() {
                                                let skill = &skills[i];
                                                match skill.name {
                                                    "JavaScript" => {
                                                        if let Some(next_skill) = skills.get(i + 1) {
                                                            if next_skill.name == "TypeScript" {
                                                                groups
                                                                    .push((
                                                                        "JavaScript / TypeScript".to_string(),
                                                                        vec![i, i + 1],
                                                                        skill.hours + next_skill.hours,
                                                                    ));
                                                                i += 2;
                                                                continue;
                                                            }
                                                        }
                                                        groups.push((skill.name.to_string(), vec![i], skill.hours));
                                                        i += 1;
                                                    }
                                                    "HTML" => {
                                                        if let Some(next_skill) = skills.get(i + 1) {
                                                            if next_skill.name == "CSS" {
                                                                groups
                                                                    .push((
                                                                        "HTML / CSS".to_string(),
                                                                        vec![i, i + 1],
                                                                        skill.hours + next_skill.hours,
                                                                    ));
                                                                i += 2;
                                                                continue;
                                                            }
                                                        }
                                                        groups.push((skill.name.to_string(), vec![i], skill.hours));
                                                        i += 1;
                                                    }
                                                    "C" => {
                                                        if let Some(next_skill) = skills.get(i + 1) {
                                                            if next_skill.name == "C++" {
                                                                groups
                                                                    .push((
                                                                        "C / C++".to_string(),
                                                                        vec![i, i + 1],
                                                                        skill.hours + next_skill.hours,
                                                                    ));
                                                                i += 2;
                                                                continue;
                                                            }
                                                        }
                                                        groups.push((skill.name.to_string(), vec![i], skill.hours));
                                                        i += 1;
                                                    }
                                                    _ => {
                                                        groups.push((skill.name.to_string(), vec![i], skill.hours));
                                                        i += 1;
                                                    }
                                                }
                                            }
                                        } else {
                                            for (idx, skill) in skills.iter().enumerate() {
                                                groups.push((skill.name.to_string(), vec![idx], skill.hours));
                                            }
                                        }
                                        let max_hours = groups.iter().map(|(_, _, hours)| *hours).max().unwrap_or(1);
                                        let mut bar_graph_class = String::from("bar-graph");
                                        if section_idx == 0 {
                                            bar_graph_class.push_str(" languages");
                                        }
                                        if hovered_idx.is_some() {
                                            bar_graph_class.push_str(" hovering");
                                        }
                                        rsx! {
                                            div { class: "{bar_graph_class}",
                                                for (idx , (label , skill_indices , hours)) in groups.iter().enumerate() {
                                                    {
                                                        let percentage = ((*hours as f64 / max_hours as f64) * 100.0).round() as u32;
                                                        let is_active = hovered_idx
                                                            .map(|hover_idx| skill_indices.contains(&hover_idx))
                                                            .unwrap_or(true);
                                                        let data_skill = skill_indices
                                                            .iter()
                                                            .map(|&skill_idx| skills[skill_idx].name)
                                                            .collect::<Vec<_>>()
                                                            .join(",");
                                                        rsx! {
                                                            div {
                                                                class: "bar-item",
                                                                "data-active": if is_active { "true" } else { "false" },
                                                                key: "bar-{idx}-{label}",
                                                                div { class: "bar-wrapper",
                                                                    div {
                                                                        class: "bar-fill",
                                                                        "data-skill": "{data_skill}",
                                                                        style: "height: {percentage}%;",
                                                                    }
                                                                    span { class: "bar-hours", "{hours}h" }
                                                                }
                                                                span { class: "bar-label", "{label}" }
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

                    div { class: "dots",
                        for i in 0..skill_sections.len() {
                            button {
                                class: if i == current_section() { "dot active" } else { "dot" },
                                onclick: move |_| {
                                    let cur = current_section();
                                    if i > cur {
                                        slide_direction.set("slide-right");
                                    } else {
                                        slide_direction.set("slide-left");
                                    }
                                    current_section.set(i);
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
            p { class: "click-hint", "Click to expand" }
            div { class: "projects-row",
                {
                    let projects_read = projects.read();
                    let featured: Vec<_> = projects_read.iter().filter(|p| p.is_featured).collect();
                    let total = featured.len();

                    rsx! {
                        for (i , project) in featured.into_iter().enumerate() {
                            {
                                let id = project.id;
                                let is_expanded = expanded_project().as_deref() == Some(id);
                                let is_faded = expanded_project().is_some() && !is_expanded;

                                rsx! {
                                    div {
                                        class: if is_expanded { "featured-card expanded" } else if is_faded { "featured-card faded" } else { "featured-card" },
                                        style: "--glow-color: {project.glow_color}; --i: {i}; --total: {total};",
                                        onclick: toggle_project(id),

                                        // Brief Content
                                        div { class: "project-content-brief",
                                            h3 { "{project.name}" }
                                            p { class: "brief", "{project.brief}" }
                                        }

                                        // Full Content
                                        div { class: "project-content-full",
                                            h3 { "{project.name}" }

                                            for para in &project.detailed_description {
                                                p { class: "detail", "{para}" }
                                            }

                                            div { class: "tags",
                                                for tech in &project.technologies {
                                                    span { "{tech}" }
                                                }
                                            }

                                            if let Some(url) = project.github_url {
                                                a { href: "{url}", target: "_blank", class: "link", "GitHub ↗" }
                                            }

                                            if let Some(url) = project.demo_url {
                                                a { href: "{url}", target: "_blank", class: "link", "Live Demo ↗" }
                                            }

                                            if project.private {
                                                p { class: "note", "Private Repository" }
                                            }

                                            p { class: "collapse-hint", "Click to collapse" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // CAREER -------------------------------------------------------------
        section { id: "career-section", class: "home-section career-pane",
            h2 { "Career" }
            div { class: "timeline glass-bw",
                div { class: "job",
                    h3 { "Full Stack Developer" }
                    p { class: "meta", "NexSys • 2025 - Present" }
                    p { "Rust + Tauri + Dioxus apps, CI/CD, agile collaboration." }
                }
                div { class: "job",
                    h3 { "Software Engineer" }
                    p { class: "meta", "Freelance • 2022 - Present" }
                    p { "Delivered solo full‑stack & tooling projects solving practical problems." }
                }
                div { class: "job",
                    h3 { "Backend Infrastructure Engineer" }
                    p { class: "meta", "Mind Over Machines • 2024" }
                    p { "Optimized client systems & accelerated development pathways." }
                }
                div { class: "job",
                    h3 { "Backend Engineer" }
                    p { class: "meta", "Gulp Data • 2023" }
                    p { "Python backend maintenance, AI search improvements." }
                }
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
                                a {
                                    href: "https://github.com/GitanElyon",
                                    target: "_blank",
                                    "GitHub ↗"
                                }
                                a {
                                    href: "https://linkedin.com/in/gitaneylon",
                                    target: "_blank",
                                    "LinkedIn"
                                }
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
                                form { class: "contact-form", onsubmit: handle_submit,

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
                            },
                        }
                    }
                }
            }
        }
    }
}
