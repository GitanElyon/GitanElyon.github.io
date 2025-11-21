use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let script = r#"
        (function() {
            const navbar = document.getElementById('navbar');
            let lastScrollTop = 0;
            if (navbar) {
                window.addEventListener('scroll', () => {
                    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
                    if (scrollTop > lastScrollTop && scrollTop > 0) {
                        navbar.classList.add('navbar-hidden');
                    } else {
                        navbar.classList.remove('navbar-hidden');
                    }
                    lastScrollTop = scrollTop <= 0 ? 0 : scrollTop;
                });
            }
        })();
    "#;

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

        script {
            dangerous_inner_html: "{script}"
        }

        Outlet::<Route> {}
    }
}
