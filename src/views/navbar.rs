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

#[component]
pub fn Navbar() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let Some(window) = web_sys::window() else { return };
            let Some(document) = window.document() else { return };
            let last_scroll = Rc::new(Cell::new(window.scroll_y().unwrap_or(0.0).max(0.0)));
            let is_hidden = Rc::new(Cell::new(false));
            let ls = last_scroll.clone();
            let ih = is_hidden.clone();
            let w = window.clone();
            let d = document.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
                let y = w.scroll_y().unwrap_or(0.0);
                let delta = y - ls.get();
                let Some(el) = d.get_element_by_id("navbar") else { ls.set(y.max(0.0)); return };
                let Ok(nav) = el.dyn_into::<web_sys::HtmlElement>() else { ls.set(y.max(0.0)); return };

                if y <= 16.0 {
                    let _ = nav.class_list().remove_1("navbar-hidden");
                    ih.set(false);
                } else if delta > 2.0 && y > 72.0 && !ih.get() {
                    let _ = nav.class_list().add_1("navbar-hidden");
                    ih.set(true);
                } else if delta < -2.0 && ih.get() {
                    let _ = nav.class_list().remove_1("navbar-hidden");
                    ih.set(false);
                }
                ls.set(y.max(0.0));
            });

            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }

    let mut menu_open = use_signal(|| false);

    rsx! {
        if menu_open() {
            div { class: "nav-backdrop", onclick: move |_| menu_open.set(false) }
        }

        nav { id: "navbar", class: if menu_open() { "menu-open" } else { "" },
            div { class: "nav-brand",
                Link { to: Route::Home {}, "Gitan Elyon" }
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
