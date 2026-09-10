use crate::data::profile::{
    get_profile_data, ProfileContactItem, ProfileEntry, ProfilePublication, ProfileSkillCategory,
};
use dioxus::prelude::*;

fn trigger_print() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = web_sys::window() {
            let _ = win.print();
        }
    }
}

fn render_bullets(entry: &ProfileEntry) -> Element {
    rsx! {
        ul {
            for bullet in &entry.bullets {
                li { "{bullet}" }
            }
        }
    }
}

fn render_skill_category(category: &ProfileSkillCategory) -> Element {
    rsx! {
        div { class: "skill-category",
            h4 { "{category.title}" }
            p { "{category.skills.join(\", \")}" }
        }
    }
}

fn render_education(school: &str, detail: &str, extra: &str) -> Element {
    rsx! {
        div { class: "resume-item",
            h3 { "{school}" }
            p { class: "company", "{detail}" }
            if !extra.is_empty() {
                p { class: "company", "{extra}" }
            }
        }
    }
}

fn render_entry(entry: &ProfileEntry) -> Element {
    rsx! {
        div { class: "resume-item",
            h3 { "{entry.title}" }
            p { class: "meta", "{entry.meta}" }
            {render_bullets(entry)}
        }
    }
}

fn render_publication(item: &ProfilePublication) -> Element {
    rsx! {
        div { class: "resume-item",
            h3 { "{item.title}" }
            p { class: "meta", "{item.detail}" }
            p { "{item.description}" }
            if let Some(href) = &item.link {
                a {
                    href: "{href}",
                    target: "_blank",
                    rel: "noopener",
                    class: "link",
                    "{href}"
                }
            }
        }
    }
}

fn render_contact_item(item: &ProfileContactItem) -> Element {
    rsx! {
        div { class: "contact-method",
            h4 { "{item.label}" }
            if let Some(href) = &item.href {
                a {
                    href: "{href}",
                    target: if href.starts_with("http") { "_blank" } else { "_self" },
                    rel: "noopener",
                    "{item.value}"
                }
            } else {
                p { "{item.value}" }
            }
        }
    }
}

#[component]
pub fn Profile() -> Element {
    let data = get_profile_data();
    let on_download = move |_| trigger_print();

    rsx! {
        document::Title { "Profile — Gitan Elyon" }
        section { id: "profile",
            div { class: "container",
                h1 { class: "page-title", "Profile" }
                div { class: "resume-content",
                    div { class: "resume-section resume-header",
                        h1 { "{data.full_name}" }
                        p { class: "resume-title", "{data.title}" }
                    }

                    div { class: "resume-section",
                        h2 { "Summary" }
                        p { "{data.summary}" }
                    }

                    div { class: "resume-section",
                        h2 { "Professional Experience" }
                        for entry in &data.experience {
                            {render_entry(entry)}
                        }
                    }

                    div { class: "resume-section",
                        h2 { "Technical Projects" }
                        for entry in &data.projects {
                            {render_entry(entry)}
                        }
                    }

                    if !data.publications.is_empty() {
                        div { class: "resume-section",
                            h2 { "Publications" }
                            for item in &data.publications {
                                {render_publication(item)}
                            }
                        }
                    }

                    div { class: "resume-section",
                        h2 { "Education" }
                        {render_education(
                            &data.school,
                            &format!("{} {}", data.degree, data.major),
                            &data.graduation,
                        )}
                    }

                    div { class: "resume-section",
                        h2 { "Technical Skills" }
                        div { class: "skills-categories",
                            for category in &data.skills {
                                {render_skill_category(category)}
                            }
                        }
                    }

                    div { class: "resume-section",
                        h2 { "Contact" }
                        div { class: "contact-grid",
                            for item in &data.contact_items {
                                {render_contact_item(item)}
                            }
                        }
                    }

                    div { class: "resume-actions",
                        button { class: "btn btn-primary", onclick: on_download, "Print / Save PDF" }
                        a {
                            href: "https://gitanelyon.dev/resume/",
                            target: "_blank",
                            rel: "noopener",
                            class: "btn btn-secondary",
                            "Printable Resume ↗"
                        }
                    }
                }
            }
        }
    }
}
