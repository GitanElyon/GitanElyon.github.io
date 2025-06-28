use dioxus::prelude::*;
use serde::Serialize;

// Define the different states our form can be in
#[derive(Clone, PartialEq)]
enum FormStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

// A struct to hold our form data, which we can serialize to JSON
#[derive(Clone, Default, Serialize)]
struct FormData {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn Contact() -> Element {
    let mut form_data = use_signal(FormData::default);
    let mut status = use_signal(|| FormStatus::Idle);

    let handle_submit = move |_| {
        // Set the status to Submitting to show loading feedback
        status.set(FormStatus::Submitting);

        // Spawn an async task to send the data
        spawn(async move {
            let formspree_url = "https://formspree.io/f/mldnapzn"; // Replace with your Formspree ID

            let client = reqwest::Client::new();
            let response = client
                .post(formspree_url)
                .header("Accept", "application/json") // Important: This tells Formspree to send a JSON response instead of redirecting
                .json(&form_data.read().clone())
                .send()
                .await;

            match response {
                Ok(res) if res.status().is_success() => {
                    status.set(FormStatus::Success);
                    form_data.set(FormData::default()); // Clear the form on success
                }
                Ok(res) => {
                    let error_text = res.text().await.unwrap_or_else(|_| "An unknown error occurred.".to_string());
                    status.set(FormStatus::Error(format!("Failed to send message: {}", error_text)));
                }
                Err(err) => {
                    status.set(FormStatus::Error(format!("Network error: {}", err)));
                }
            }
        });
    };

    rsx! {
        section {
            id: "contact",
            div {
                class: "container",
                h1 { "Get In Touch" }
                div {
                    class: "contact-content",
                    div {
                        class: "contact-info",
                        // ... your existing contact info ...
                        h2 { "Let's Connect" }
                        p {
                            "I'm always interested in hearing about new opportunities, 
                            exciting projects, or just having a chat about technology."
                        }
                        
                        div {
                            class: "contact-methods",
                            div {
                                class: "contact-method",
                                h3 { "Email" }
                                p { "your-email@example.com" }
                            }
                            div {
                                class: "contact-method",
                                h3 { "Phone" }
                                p { "(555) 123-4567" }
                            }
                            div {
                                class: "contact-method",
                                h3 { "Location" }
                                p { "City, State, Country" }
                            }
                        }

                        div {
                            class: "social-links",
                            h3 { "Follow Me" }
                            div {
                                class: "social-icons",
                                a { href: "https://github.com/GitanElyon", target: "_blank", "GitHub" }
                                a { href: "https://linkedin.com/in/yourprofile", target: "_blank", "LinkedIn" }
                                a { href: "https://twitter.com/yourhandle", target: "_blank", "Twitter" }
                            }
                        }
                    }

                    div {
                        class: "contact-form-container",
                        // Conditionally render based on the form status
                        match status() {
                            FormStatus::Success => rsx! {
                                div {
                                    class: "success-message",
                                    h3 { "Message Sent!" }
                                    p { "Thanks for reaching out. I'll get back to you soon." }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| status.set(FormStatus::Idle),
                                        "Send Another"
                                    }
                                }
                            },
                            FormStatus::Error(error_msg) => rsx! {
                                div {
                                    class: "error-message",
                                    h3 { "Something Went Wrong" }
                                    p { "{error_msg}" }
                                    button {
                                        class: "btn btn-secondary",
                                        onclick: move |_| status.set(FormStatus::Idle),
                                        "Try Again"
                                    }
                                }
                            },
                            _ => rsx! { // Idle and Submitting states
                                form {
                                    class: "contact-form",
                                    prevent_default: "onsubmit",
                                    onsubmit: handle_submit,
                                    
                                    div {
                                        class: "form-group",
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
                                    
                                    div {
                                        class: "form-group",
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
                                    
                                    div {
                                        class: "form-group",
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
                            }
                        }
                    }
                }
            }
        }
    }
}