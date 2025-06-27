use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav {
            id: "navbar",
            div {
                class: "nav-brand",
                Link { to: Route::Home {}, "Gitan Elyon" }
            }
            div {
                class: "nav-links",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::About {}, "About" }
                Link { to: Route::Projects {}, "Projects" }
                Link { to: Route::Resume {}, "Resume" }
                Link { to: Route::Contact {}, "Contact" }
            }
        }

        Outlet::<Route> {}
    }
}