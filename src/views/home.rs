use crate::components::{Hero, SkillBadge};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
            Hero {}

            section {
                id: "skills-preview",
                div {
                    class: "container",
                    h2 { "Technical Skills" }

                    div {
                        class: "skills-section",
                        h3 { "Programming Languages" }
                        div {
                            class: "skills-grid",
                            SkillBadge { name: "Rust".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Go".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Python".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "JavaScript".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "TypeScript".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "HTML/CSS".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "Java".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "C/C++".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "C#".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "Lua".to_string(), level: "Beginner".to_string() }
                        }
                    }

                    div {
                        class: "skills-section",
                        h3 { "Frameworks & Libraries" }
                        div {
                            class: "skills-grid",
                            SkillBadge { name: "Tauri".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Bun.js".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Dioxus".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "Node.js".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "Svelte".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "Electron".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "Vue".to_string(), level: "Beginner".to_string() }
                            SkillBadge { name: "React".to_string(), level: "Beginner".to_string()
                        }
                    }

                    div {
                        class: "skills-section",
                        h3 { "Databases & Search" }
                        div {
                            class: "skills-grid",
                            SkillBadge { name: "Solr".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "SurrealDB".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "SEO".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "PostgreSQL".to_string(), level: "Beginner".to_string() }
                        }
                    }

                    div {
                        class: "skills-section",
                        h3 { "Tools & Systems" }
                        div {
                            class: "skills-grid",
                            SkillBadge { name: "Linux".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Git".to_string(), level: "Expert".to_string() }
                            SkillBadge { name: "Nix".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "Agile".to_string(), level: "Advanced".to_string() }
                            SkillBadge { name: "Adobe Suite".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "IDA Pro".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "CI/CD".to_string(), level: "Intermediate".to_string() }
                            SkillBadge { name: "Ghidra".to_string(), level: "Beginner".to_string() }
                        }
                    }
                }
            }
        }
    }
}
