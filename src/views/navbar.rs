use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let Some(window) = web_sys::window() else { return };
            let Some(document) = window.document() else { return };
            
            let mut last_scroll = window.scroll_y().unwrap_or(0.0).max(0.0);
            let mut is_hidden = false;

            // Use requestAnimationFrame-based throttle for better performance
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            
            let window_clone = window.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let y = window_clone.scroll_y().unwrap_or(0.0);
                let delta = y - last_scroll;
                
                if let Some(nav) = document
                    .get_element_by_id("navbar")
                    .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                {
                    if y <= 16.0 {
                        let _ = nav.class_list().remove_1("navbar-hidden");
                        is_hidden = false;
                    } else if delta > 2.0 && y > 72.0 && !is_hidden {
                        let _ = nav.class_list().add_1("navbar-hidden");
                        is_hidden = true;
                    } else if delta < -2.0 && is_hidden {
                        let _ = nav.class_list().remove_1("navbar-hidden");
                        is_hidden = false;
                    }
                }
                last_scroll = y.max(0.0);
            });

            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }

    rsx! {
        if menu_open() {
            div { class: "nav-backdrop", onclick: move |_| menu_open.set(false) }
        }

        nav { id: "navbar", class: if menu_open() { "menu-open" } else { "" },
            div { class: "nav-brand",
                Link {
                    to: Route::Home {},
                    onclick: move |_| menu_open.set(false),
                    "Gitan Elyon"
                }
            }
            button {
                class: if menu_open() { "hamburger open" } else { "hamburger" },
                onclick: move |_| menu_open.set(!menu_open()),
                "aria-label": "Toggle navigation menu",
                span { class: "hamburger-line" }
                span { class: "hamburger-line" }
                span { class: "hamburger-line" }
            }
            div { class: if menu_open() { "nav-links open" } else { "nav-links" },
                Link {
                    to: Route::Home {},
                    onclick: move |_| menu_open.set(false),
                    "Home"
                }
                Link {
                    to: Route::About {},
                    onclick: move |_| menu_open.set(false),
                    "About"
                }
                Link {
                    to: Route::Projects {},
                    onclick: move |_| menu_open.set(false),
                    "Projects"
                }
                Link {
                    to: Route::Resume {},
                    onclick: move |_| menu_open.set(false),
                    "Resume"
                }
                Link {
                    to: Route::Contact {},
                    onclick: move |_| menu_open.set(false),
                    "Contact"
                }
            }
        }

        Outlet::<Route> {}
    }
}
