use dioxus::prelude::*;
use crate::Route;

const PROFILE_IMAGE: Asset = asset!("/assets/profile.jpg");

#[component]
pub fn Hero() -> Element {
    rsx! {
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
    }
}