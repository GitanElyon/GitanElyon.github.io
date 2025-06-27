use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProjectCardProps {
    title: String,
    description: String,
    technologies: Vec<String>,
    github_url: Option<String>,
    demo_url: Option<String>,
    image_url: Option<String>,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    rsx! {
        div {
            class: "project-card",
            if let Some(image) = &props.image_url {
                div {
                    class: "project-image",
                    img { src: "{image}", alt: "{props.title}" }
                }
            }
            div {
                class: "project-content",
                h3 { "{props.title}" }
                p { "{props.description}" }
                div {
                    class: "tech-stack",
                    for tech in &props.technologies {
                        span { class: "tech-tag", "{tech}" }
                    }
                }
                div {
                    class: "project-links",
                    if let Some(github) = &props.github_url {
                        a { href: "{github}", target: "_blank", "GitHub" }
                    }
                    if let Some(demo) = &props.demo_url {
                        a { href: "{demo}", target: "_blank", "Live Demo" }
                    }
                }
            }
        }
    }
}