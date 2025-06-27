use crate::components::ProjectCard;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        section {
            id: "projects",
            div {
                class: "container",
                h1 { "My Projects" }
                div {
                    class: "projects-grid",
                    ProjectCard {
                        title: "Loginsight".to_string(),
                        description: "A lightweight and modern desktop app to help Windows users identify computer issues and speed up their computers.".to_string(),
                        technologies: vec!["Tauri".to_string(), "Rust".to_string(), "HTML".to_string(), "CSS".to_string(), "JavaScript".to_string()],
                        github_url: Some("https://github.com/GitanElyon/loginsight".to_string()),
                        image_url: None,
                    }
                    ProjectCard {
                        title: "YAWMA".to_string(),
                        description: "Yet Another Workload Managment App is a tool I built to help keep on tast with my dailey work life using git, cloud storage and other tools to help users track their work.".to_string(),
                        technologies: vec!["Tauri".to_string(), "Rust".to_string(), "HTML".to_string(), "CSS".to_string(), "JavaScript".to_string()],
                        github_url: Some("https://github.com/GitanElyon/YAWMA".to_string()),
                        demo_url: None,
                        image_url: None,
                    }
                    ProjectCard {
                        title: "Catalyst".to_string(),
                        description: "A super minimal and lightweight code editor designed to expide the process of idea to product with no distractions".to_string(),
                        technologies: vec!["Tauri".to_string(), "Rust".to_string(), "Dioxus".to_string(), "CSS".to_string()],
                        github_url: Some("https://github.com/GitanElyon/catalyst".to_string()),
                        demo_url: None,
                        image_url: None,
                    }
                    ProjectCard {
                        title: "Papyr".to_string(),
                        description: "Not quite ready to show it off yet, buts going to be gamechanging.".to_string(),
                        technologies: vec!["Rust".to_string(), "Svelte".to_string(), "CSS".to_string()],
                        github_url: Some("https://github.com/GitanElyon/catalyst".to_string()),
                        demo_url: None,
                        image_url: None,
                    }
                    ProjectCard {
                        title: "Minos".to_string(),
                        description: "An AI designed to play tetris perfectly in real time using python to read the screen and rust for calculations".to_string(),
                        technologies: vec!["Rust".to_string(), "Python".to_string(), "AI".to_string(), "Machine Learning".to_string()],
                        github_url: Some("https://github.com/GitanElyon/minos".to_string()),
                        demo_url: None,
                        image_url: None,
                    }
                    ProjectCard {
                        title: "Blackjack AI".to_string(),
                        description: "An AI built to play blackjack as optimally as possible.".to_string(),
                        technologies: vec!["Rust".to_string(), "AI".to_string(), "Machine Learning".to_string()],
                        github_url: Some("https://github.com/GitanElyon/blackjack-ai".to_string()),
                        demo_url: None,
                        image_url: None,
                    }
                    /*ProjectCard {
                        title: "Example Project 2".to_string(),
                        description: "Description of another project you've worked on. Add real projects here.".to_string(),
                        technologies: vec!["TypeScript".to_string(), "React".to_string(), "Node.js".to_string()],
                        github_url: Some("https://github.com/GitanElyon".to_string()),
                        demo_url: None,
                        image_url: None,
                    }*/
                    ProjectCard {
                        title: "Portfolio Website".to_string(),
                        description: "A modern portfolio website built with Dioxus and Rust, showcasing responsive design and WebAssembly performance.".to_string(),
                        technologies: vec!["Rust".to_string(), "Dioxus".to_string(), "WebAssembly".to_string(), "CSS".to_string()],
                        github_url: Some("https://github.com/GitanElyon/gitanelyon.github.io".to_string()),
                        demo_url: Some("https://gitanelyon.github.io".to_string()),
                        image_url: None,
                    }
                }
            }
        }
    }
}