use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        section {
            id: "resume",
            div {
                class: "container",
                h1 { "Resume" }
                div {
                    class: "resume-content",
                    
                    // Header Section
                    div {
                        class: "resume-section",
                        h2 { "Gitan Elyon Mandell-Balogh" }
                        p { class: "resume-title", "Software Engineer & Full-Stack Developer" }
                        p { class: "resume-contact", 
                            "Email: gitanelyon@gmail.com | Phone: (443) 224-8540 | Location: Pikesville Baltimore"
                        }
                    }

                    // Experience Section
                    div {
                        class: "resume-section",
                        h2 { "Professional Experience" }
                        div {
                            class: "resume-item",
                            h3 { "Full Stack Developer" }
                            p { class: "company", "NexSys | May 2025 - Present" }
                            ul {
                                li { "Built responsive web applications using Rust, Tauri and Dioxus" }
                                li { "Collaborated with cross-functional teams in Agile environment" }
                                li { "Implemented CI/CD pipelines and automated testing" }
                            }
                        }
                        div {
                            class: "resume-item",
                            h3 { "Software Engineer" }
                            p { class: "company", "Freelancing | 2022 - Present" }
                            ul {
                                li { "Built lightweight applications using Javascript and Rust." }
                                li { "Worked as a solo developer on a wide varity of tasks" }
                                li { "Discovered problems and solutions for problems in everyday life" }
                            }
                        }
                        div {
                            class: "resume-item",
                            h3 { "Backend Infrastructure Engineer" }
                            p { class: "company", "Mind Over Machines | May 2024 - September 2024" }
                            ul {
                                li { "Optimized digital applications and systems for clients" }
                                li { "Worked with engineers of all levels in a fast paced enviroment" }
                                li { "Implemented systems to accelerate technology development" }
                            }
                        }
                        div {
                            class: "resume-item",
                            h3 { "Backend Engineer" }
                            p { class: "company", "Gulp Data | May 2023 - September 2023" }
                            ul {
                                li { "Developed and maintained website backend using python" }
                                li { "Taught by and worked with senior software engineers" }
                                li { "Improved QA page by using AI to searching alorithms" }
                            }
                        }
                    }

                    // Education Section
                    div {
                        class: "resume-section",
                        h2 { "Education" }
                        div {
                            class: "resume-item",
                            h3 { "Bachelor of Engineering in Computer Engineering" }
                            p { class: "company", "University of Maryland | 2024 - Current" }
                            p { "Relevant Coursework: Data Structures, Algorithms, Software Engineering, Database Systems" }
                        }
                    }

                    // Skills Section
                    div {
                        class: "resume-section",
                        h2 { "Technical Skills" }
                        div {
                            class: "skills-categories",
                            div {
                                class: "skill-category",
                                h4 { "Programming Languages" }
                                p { "Rust, Go, Python, JavaScript, TypeScript, Java, Lua, C/C++, C#, HTML/CSS" }
                            }
                            div {
                                class: "skill-category",
                                h4 { "Frameworks & Libraries" }
                                p { "Node.js, Svelte, Dioxus, Vue, Bun.js, Electron, Tauri" }
                            }
                            div {
                                class: "skill-category",
                                h4 { "Databases & Search" }
                                p { "PostgreSQL, Solr, SurrealDB, SEO" }
                            }
                            div {
                                class: "skill-category",
                                h4 { "Tools & Systems" }
                                p { "Linux, Nix, Git, Adobe Suite, IDA Pro, Ghidra, CI/CD, Agile Methodologies" }
                            }
                        }
                    }

                    // Download Button
                    div {
                        class: "resume-actions",
                        a {
                            href: "#",
                            class: "btn btn-primary",
                            "Download PDF Resume"
                        }
                    }
                }
            }
        }
    }
}