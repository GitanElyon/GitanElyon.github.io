use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about",
            div {
                class: "container",
                h1 { "About Me" }
                div {
                    class: "about-content",
                    div {
                        class: "about-text",
                        p {
                            "I'm a passionate software engineer with a strong background in full-stack development. 
                            I love working with cutting-edge technologies and building applications that make a difference."
                        }
                        p {
                            "My journey in software development started with a desire to find solutions to problems I find in everyday life. 
                            Over the years, I've honed my skills in various programming languages and frameworks, while maintaining a focus on performance and user experience."
                        }
                        h2 { "What I Do" }
                        ul {
                            li { "Full-stack web application development" }
                            li { "Systems programming and performance optimization" }
                            li { "Freelancing for anyone who has an idea" }
                            li { "Open source contributions" }
                        }
                        h2 { "Main Technologies I Use" }
                        ul {
                            li { "Rust - My goto language for building high-performance applications" }
                            li { "JavaScript - Its not my favorite to work in, but if I can dream of it, JS can make it happen" }
                            li { "Go - Most of my freelancing work was done using Go because of simple yet fast it is" }
                            li { "Python - My typical choice when just prototyping, building a quick script or teaching a friend to code" }
                        }
                        h2 { "When I'm Not Coding" }
                        p {
                            "When I'm not coding, I enjoy rock climbing, studying the newest technologies and building all sorts of little computers.
                            I love spending time with friends and family, exploring new places, and learning about different cultures."
                        }
                        p {
                            "I'm always looking for new challenges and opportunities to grow as a developer. 
                            If you have an interesting project or just want to chat about technology, feel free to reach out!"
                        }
                    }
                }
            }
        }
    }
}