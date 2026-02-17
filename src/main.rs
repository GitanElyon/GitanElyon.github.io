// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::Background;
use views::{About, Contact, Home, Navbar, Projects, Resume};

/// Define a components module that contains all shared components for our app.
mod components;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;
/// Define a data module that contains the static data for our app.
mod data;

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
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

// Assets
const FAVICON: Asset = asset!("/assets/favicon.ico");
// Removed MAIN_CSS and replaced with individual global styles
const BASE_CSS: Asset = asset!("/assets/styling/base.css");
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
        // Global stylesheets
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: BACKGROUND_CSS }
        document::Link { rel: "stylesheet", href: RESPONSIVE_CSS }
        Background {}
        Router::<Route> {}
    }
}
