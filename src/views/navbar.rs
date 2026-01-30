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
            let Some(nav_element) = document.get_element_by_id("navbar") else {
                return;
            };
            let Ok(nav) = nav_element.dyn_into::<web_sys::HtmlElement>() else {
                return;
            };

            let last_scroll = Rc::new(Cell::new(0.0));
            let last_scroll_clone = last_scroll.clone();
            let nav_clone = nav.clone();
            let window_clone = window.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
                let scroll_top = window_clone.scroll_y().unwrap_or(0.0);
                if scroll_top > last_scroll_clone.get() && scroll_top > 0.0 {
                    let _ = nav_clone.class_list().add_1("navbar-hidden");
                } else {
                    let _ = nav_clone.class_list().remove_1("navbar-hidden");
                }
                last_scroll_clone.set(scroll_top.max(0.0));
            });

            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        nav { id: "navbar",
            div { class: "nav-brand",
                Link { to: Route::Home {}, "Gitan Elyon" }
            }
            div { class: "nav-links",
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
