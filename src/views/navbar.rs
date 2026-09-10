use crate::Route;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use std::sync::atomic::{AtomicBool, Ordering};

/// The scroll listener is attached to `window` (global), so it must only be
/// added once no matter how often the Navbar re-renders or remounts.
#[cfg(target_arch = "wasm32")]
static NAV_SCROLL_LISTENER: AtomicBool = AtomicBool::new(false);

#[component]
pub fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            // Bail out if a previous mount already installed the listener.
            if NAV_SCROLL_LISTENER.swap(true, Ordering::SeqCst) {
                return;
            }
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };

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

            let _ =
                window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
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
                "aria-expanded": "{menu_open()}",
                "aria-controls": "nav-links",
                onkeydown: move |e| {
                    if e.key() == Key::Escape {
                        menu_open.set(false);
                    }
                },
                span { class: "hamburger-line" }
                span { class: "hamburger-line" }
                span { class: "hamburger-line" }
            }
            div { class: if menu_open() { "nav-links open" } else { "nav-links" }, id: "nav-links",
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
                    to: Route::Profile {},
                    onclick: move |_| menu_open.set(false),
                    "Profile"
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
