use crate::components::ContactForm;
use crate::data::profile::get_profile_data;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let profile = get_profile_data();

    rsx! {
        document::Title { "Contact — Gitan Elyon" }
        section { id: "contact",
            div { class: "container",
                h1 { class: "page-title", "Get In Touch" }
                div { class: "contact-content",
                    div { class: "contact-info",
                        h2 { "Let's Connect" }
                        p {
                            "I'm always interested in hearing about new opportunities, exciting projects, or just having a chat about technology."
                        }
                        div { class: "contact-methods",
                            div { class: "contact-method",
                                h3 { "Name" }
                                p { "{profile.full_name}" }
                            }
                            div { class: "contact-method",
                                h3 { "Email" }
                                p {
                                    a { href: "mailto:{profile.email}", "{profile.email}" }
                                }
                            }
                            div { class: "contact-method",
                                h3 { "Location" }
                                p { "{profile.location}" }
                            }
                        }
                        div { class: "social-links",
                            h3 { "Follow Me" }
                            div { class: "social-icons",
                                a {
                                    href: "{profile.github}",
                                    target: "_blank",
                                    rel: "noopener me",
                                    "GitHub"
                                }
                                a {
                                    href: "{profile.linkedin}",
                                    target: "_blank",
                                    rel: "noopener me",
                                    "LinkedIn"
                                }
                            }
                        }
                    }
                    div { class: "contact-form-container",
                        ContactForm {}
                    }
                }
            }
        }
    }
}
