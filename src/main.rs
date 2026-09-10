use dioxus::prelude::*;

use components::Background;
use views::{About, Contact, Home, Links, Navbar, Profile, Projects};

mod components;
mod data;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/projects")]
        Projects {},
        #[route("/profile")]
        Profile {},
        #[route("/contact")]
        Contact {},
        #[end_layout]
        // Standalone pages: no navbar.
        #[route("/links")]
        Links {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

const SITE_URL: &str = "https://gitanelyon.dev/";
const SITE_DESCRIPTION: &str = "Gitan Elyon Mandell-Balogh — Systems Engineer building high-performance applications with Rust, Go, and modern web tools.";
const FAVICON: Asset = asset!("/assets/favicon.ico");
const HEADSHOT_META: Asset = asset!("/assets/profile.webp");
const BASE_CSS: Asset = asset!("/assets/styling/base.css");
const COMPONENTS_CSS: Asset = asset!("/assets/styling/components.css");
const BACKGROUND_CSS: Asset = asset!("/assets/styling/background.css");
const RESPONSIVE_CSS: Asset = asset!("/assets/styling/responsive.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "canonical", href: SITE_URL }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes",
        }
        document::Meta { name: "description", content: SITE_DESCRIPTION }
        document::Meta { name: "author", content: "Gitan Elyon Mandell-Balogh" }
        document::Meta { name: "theme-color", content: "#0a0e1a" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Gitan Elyon" }
        document::Meta { property: "og:url", content: SITE_URL }
        document::Meta { property: "og:title", content: "Gitan Elyon — Systems Engineer" }
        document::Meta { property: "og:description", content: SITE_DESCRIPTION }
        document::Meta { property: "og:image", content: HEADSHOT_META }
        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:title", content: "Gitan Elyon — Systems Engineer" }
        document::Meta { name: "twitter:description", content: SITE_DESCRIPTION }
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }
        document::Link { rel: "stylesheet", href: BACKGROUND_CSS }
        document::Link { rel: "stylesheet", href: RESPONSIVE_CSS }
        Background {}
        Router::<Route> {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let path = format!("/{}", segments.join("/"));
    rsx! {
        document::Title { "Not Found — Gitan Elyon" }
        section { id: "not-found",
            div { class: "container",
                h1 { class: "page-title", "Page Not Found" }
                div { class: "glass not-found-card",
                    p { "No page exists at “{path}”." }
                    div { class: "hero-buttons",
                        Link { to: Route::Home {}, class: "btn btn-primary", "Back Home" }
                        Link { to: Route::Profile {}, class: "btn btn-secondary", "View Profile" }
                    }
                }
            }
        }
    }
}
