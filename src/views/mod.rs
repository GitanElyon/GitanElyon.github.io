//! The views module contains the components for all Layouts and Routes for our app. Each layout and route in our [`Route`]
//! enum will render one of these components.
//!
//!
//! The [`Home`] and [`Blog`] components will be rendered when the current route is [`Route::Home`] or [`Route::Blog`] respectively.
//!
//!
//! The [`Navbar`] component will be rendered on all pages of our app since every page is under the layout. The layout defines
//! a common wrapper around all child routes.

mod about;
mod contact;
mod home;
mod navbar;
mod projects;
mod resume;

pub use about::About;
pub use contact::Contact;
pub use home::Home;
pub use navbar::Navbar;
pub use projects::Projects;
pub use resume::Resume;
