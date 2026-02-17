use crate::Route;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use std::cell::Cell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let Some(window) = web_sys::window() else {
                return;
            };
            let Some(document) = window.document() else {
                return;
            };
            let last_scroll = Rc::new(Cell::new(window.scroll_y().unwrap_or(0.0).max(0.0)));
            let is_hidden = Rc::new(Cell::new(false));
            let last_scroll_clone = last_scroll.clone();
            let is_hidden_clone = is_hidden.clone();
            let window_clone = window.clone();
            let document_clone = document.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
                let scroll_top = window_clone.scroll_y().unwrap_or(0.0);
                let delta = scroll_top - last_scroll_clone.get();

                let Some(nav_element) = document_clone.get_element_by_id("navbar") else {
                    last_scroll_clone.set(scroll_top.max(0.0));
                    return;
                };
                let Ok(nav) = nav_element.dyn_into::<web_sys::HtmlElement>() else {
                    last_scroll_clone.set(scroll_top.max(0.0));
                    return;
                };

                if scroll_top <= 16.0 {
                    let _ = nav.class_list().remove_1("navbar-hidden");
                    is_hidden_clone.set(false);
                } else if delta > 2.0 && scroll_top > 72.0 && !is_hidden_clone.get() {
                    let _ = nav.class_list().add_1("navbar-hidden");
                    is_hidden_clone.set(true);
                } else if delta < -2.0 && is_hidden_clone.get() {
                    let _ = nav.class_list().remove_1("navbar-hidden");
                    is_hidden_clone.set(false);
                }

                last_scroll_clone.set(scroll_top.max(0.0));
            });

            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }

    let mut menu_open = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        if menu_open() {
            div { class: "nav-backdrop", onclick: move |_| menu_open.set(false) }
        }

        nav { id: "navbar", class: if menu_open() { "menu-open" } else { "" },
            div { class: "nav-brand",
                Link { to: Route::Home {}, "Gitan Elyon" }
            }
            // Hamburger button for mobile
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
