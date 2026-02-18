use crate::views::home::{FormData, FormStatus};
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let mut form_data = use_signal(FormData::default);
    let mut status = use_signal(|| FormStatus::Idle);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        status.set(FormStatus::Submitting);
        spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .post("https://formspree.io/f/mldnapzn")
                .header("Accept", "application/json")
                .json(&form_data.read().clone())
                .send()
                .await;
            match response {
                Ok(res) if res.status().is_success() => {
                    status.set(FormStatus::Success);
                    form_data.set(FormData::default());
                }
                Ok(res) => {
                    let t = res.text().await.unwrap_or_else(|_| "Unknown error".into());
                    status.set(FormStatus::Error(format!("Failed: {t}")));
                }
                Err(e) => status.set(FormStatus::Error(format!("Network error: {e}"))),
            }
        });
    };

    rsx! {
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
                                h3 { "Email" }
                                p { "gitanelyon@gmail.com" }
                            }
                            div { class: "contact-method",
                                h3 { "Phone" }
                                p { "(443)-224-8540" }
                            }
                            div { class: "contact-method",
                                h3 { "Location" }
                                p { "Baltimore, Maryland, United States" }
                            }
                        }
                        div { class: "social-links",
                            h3 { "Follow Me" }
                            div { class: "social-icons",
                                a {
                                    href: "https://github.com/GitanElyon",
                                    target: "_blank",
                                    "GitHub"
                                }
                                a {
                                    href: "https://linkedin.com/in/gitaneylon",
                                    target: "_blank",
                                    "LinkedIn"
                                }
                                a {
                                    href: "https://instagram.com/gitanelyon",
                                    target: "_blank",
                                    "Instagram"
                                }
                            }
                        }
                    }
                    div { class: "contact-form-container",
                        match status() {
                            FormStatus::Success => rsx! {
                                div { class: "success-message",
                                    h3 { "Message Sent!" }
                                    p { "Thanks for reaching out. I'll get back to you soon." }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| status.set(FormStatus::Idle),
                                        "Send Another"
                                    }
                                }
                            },
                            FormStatus::Error(msg) => rsx! {
                                div { class: "error-message",
                                    h3 { "Something Went Wrong" }
                                    p { "{msg}" }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| status.set(FormStatus::Idle),
                                        "Try Again"
                                    }
                                }
                            },
                            _ => rsx! {
                                form { class: "contact-form", onsubmit: handle_submit,
                                    div { class: "form-group",
                                        label { r#for: "name", "Name" }
                                        input {
                                            r#type: "text",
                                            id: "name",
                                            required: true,
                                            disabled: status() == FormStatus::Submitting,
                                            value: "{form_data.read().name}",
                                            oninput: move |e| form_data.write().name = e.value(),
                                        }
                                    }
                                    div { class: "form-group",
                                        label { r#for: "email", "Email" }
                                        input {
                                            r#type: "email",
                                            id: "email",
                                            required: true,
                                            disabled: status() == FormStatus::Submitting,
                                            value: "{form_data.read().email}",
                                            oninput: move |e| form_data.write().email = e.value(),
                                        }
                                    }
                                    div { class: "form-group",
                                        label { r#for: "message", "Message" }
                                        textarea {
                                            id: "message",
                                            rows: "5",
                                            required: true,
                                            disabled: status() == FormStatus::Submitting,
                                            value: "{form_data.read().message}",
                                            oninput: move |e| form_data.write().message = e.value(),
                                        }
                                    }
                                    button {
                                        r#type: "submit",
                                        class: "btn btn-primary",
                                        disabled: status() == FormStatus::Submitting,
                                        if status() == FormStatus::Submitting {
                                            "Sending..."
                                        } else {
                                            "Send Message"
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
