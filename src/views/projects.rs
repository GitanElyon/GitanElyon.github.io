use crate::data::projects::get_projects;
use dioxus::prelude::*;

const PROJECTS_CSS: Asset = asset!("/assets/styling/projects.css");

#[component]
pub fn Projects() -> Element {
    let projects = get_projects();
    let mut selected_project_id = use_signal(|| None::<String>);

    let open_project = |id: &'static str| {
        let id_string = id.to_string();
        move |_| {
            selected_project_id.set(Some(id_string.clone()));
        }
    };

    let close_modal = move |_| {
        selected_project_id.set(None);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: PROJECTS_CSS }
        section { id: "projects",
            div { class: "container",
                h1 { "My Projects" }
                p { class: "click-hint", "Click any card for details" }

                // Grid of tiles
                div { class: "projects-list",
                    for project in &projects {
                        div {
                            class: "project-tile",
                            style: "--glow-color: {project.glow_color}",
                            onclick: open_project(project.id),

                            div { class: "project-content-brief",
                                h3 { "{project.name}" }
                                p { class: "brief", "{project.brief}" }
                            }
                        }
                    }
                }

                // Modal Overlay
                if let Some(id_str) = selected_project_id() {
                    if let Some(project) = projects.iter().find(|p| p.id == id_str) {
                        div { class: "modal-overlay", onclick: close_modal, // Close when clicking background
                            div {
                                class: "modal-content",
                                style: "--glow-color: {project.glow_color}",
                                onclick: |evt| evt.stop_propagation(), // Prevent closing when clicking content

                                button {
                                    class: "modal-close-btn",
                                    onclick: close_modal,
                                    "×"
                                }

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
                                    a {
                                        href: "{url}",
                                        target: "_blank",
                                        class: "link",
                                        "GitHub ↗"
                                    }
                                }

                                if let Some(url) = project.demo_url {
                                    a {
                                        href: "{url}",
                                        target: "_blank",
                                        class: "link",
                                        "Live Demo ↗"
                                    }
                                }

                                if project.private {
                                    p { class: "note", "Private Repository" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
