//! Shared contact form (used by the home section and the contact page).
//!
//! Single source of truth for the form endpoint, payload shape and
//! submit/status UI so the two don't drift apart.

use dioxus::prelude::*;

/// Formspree endpoint that receives contact submissions.
pub const FORMSPREE_URL: &str = "https://formspree.io/f/mldnapzn";

#[derive(Clone, Default, serde::Serialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[component]
pub fn ContactForm() -> Element {
    let mut form_data = use_signal(FormData::default);
    let mut status = use_signal(|| FormStatus::Idle);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        status.set(FormStatus::Submitting);
        // Clone the payload up front so no signal guard is held across `.await`.
        let payload = form_data.read().clone();
        spawn(async move {
            let client = reqwest::Client::new();
            let response = client
                .post(FORMSPREE_URL)
                .header("Accept", "application/json")
                .json(&payload)
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

    let submitting = status() == FormStatus::Submitting;

    rsx! {
        match status() {
            FormStatus::Success => rsx! {
                div { class: "success-message",
                    h3 { "Message Sent!" }
                    p { "Thanks for reaching out — I'll get back to you soon." }
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
                        label { r#for: "contact-name", "Name" }
                        input {
                            r#type: "text",
                            id: "contact-name",
                            name: "name",
                            autocomplete: "name",
                            required: true,
                            disabled: submitting,
                            value: "{form_data.read().name}",
                            oninput: move |e| form_data.write().name = e.value(),
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "contact-email", "Email" }
                        input {
                            r#type: "email",
                            id: "contact-email",
                            name: "email",
                            autocomplete: "email",
                            required: true,
                            disabled: submitting,
                            value: "{form_data.read().email}",
                            oninput: move |e| form_data.write().email = e.value(),
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "contact-message", "Message" }
                        textarea {
                            id: "contact-message",
                            name: "message",
                            rows: "5",
                            required: true,
                            disabled: submitting,
                            value: "{form_data.read().message}",
                            oninput: move |e| form_data.write().message = e.value(),
                        }
                    }
                    button {
                        r#type: "submit",
                        class: "btn btn-primary",
                        disabled: submitting,
                        if submitting { "Sending…" } else { "Send Message" }
                    }
                }
            },
        }
    }
}
