use dioxus::prelude::*;

use components::Background;
use views::{About, Contact, Home, Navbar, Projects, Resume};

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
        #[route("/resume")]
        Resume {},
        #[route("/contact")]
        Contact {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
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
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes",
        }
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }
        document::Link { rel: "stylesheet", href: BACKGROUND_CSS }
        document::Link { rel: "stylesheet", href: RESPONSIVE_CSS }
        Background {}
        Router::<Route> {}
    }
}
