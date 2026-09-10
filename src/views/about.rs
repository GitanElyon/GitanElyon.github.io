use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        document::Title { "About — Gitan Elyon" }
        section { id: "about",
            div { class: "container",
                h1 { class: "page-title", "About Me" }
                div { class: "about-content",
                    div { class: "glass about-card",
                        p {
                            "I'm a Systems Engineer working on high-performance backend and security infrastructure with Rust and Go — currently building network security tooling at Ridgeback Network Defence in Baltimore, MD."
                        }
                        p {
                            "My rule is simple: “Make it work. Make it right. Make it fast.” I like low-latency systems, reproducible Nix-based environments, and open source that other people can actually build on."
                        }
                    }
                    div { class: "glass about-card",
                        h2 { "What I Do" }
                        ul {
                            li { "Network security tooling and verifiable access-control systems" }
                            li { "High-performance backends, including LLM-driven search over Solr and PostgreSQL" }
                            li { "Cross-platform desktop apps with Tauri and Dioxus" }
                            li { "Open source: Qst, Minos, Papyr, and community plugins" }
                        }
                    }
                    div { class: "glass about-card",
                        h2 { "Main Technologies I Use" }
                        ul {
                            li { "Rust — my go-to for memory-safe, high-performance systems" }
                            li { "Go — my choice for simple, scalable backend services" }
                            li {
                                "Python — for prototyping, data work, and teaching friends to code"
                            }
                            li {
                                "JS/TS — not my favorite to write, but if I can dream it, JS can ship it"
                            }
                        }
                    }
                    div { class: "glass about-card",
                        h2 { "Background" }
                        p {
                            "I'm studying Information Science with a focus on Cyber Security at the University of Maryland (expected May 2028), and I'm a first-author on a University of Maryland publication about neighborhood graphs and clustering."
                        }
                        p {
                            "When I'm not coding, I enjoy rock climbing, studying new technologies, and building little computers. I run NixOS as my daily driver — reproducibility all the way down."
                        }
                    }
                }
            }
        }
    }
}
