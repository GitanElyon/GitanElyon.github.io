use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut message = use_signal(|| String::new());
    let mut submitted = use_signal(|| false);

    let handle_submit = move |_| {
        // In a real app, you'd send this data to a server
        println!("Form submitted: {} {} {}", name(), email(), message());
        submitted.set(true);
        
        // Reset form after a delay (in a real app, you'd handle this properly)
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(3_000).await;
            submitted.set(false);
            name.set(String::new());
            email.set(String::new());
            message.set(String::new());
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
                                p { "gitanelyon@gmail.com" }
                            }
                            div {
                                class: "contact-method",
                                h3 { "Phone" }
                                p { "(443) 224 8540" }
                            }
                            div {
                                class: "contact-method",
                                h3 { "Location" }
                                p { "Pikesville Maryland, United States" }
                            }
                        }

                        div {
                            class: "social-links",
                            h3 { "Follow Me" }
                            div {
                                class: "social-icons",
                                a { href: "https://github.com/GitanElyon", target: "_blank", "GitHub" }
                                a { href: "https://linkedin.com/in/GitanElyon", target: "_blank", "LinkedIn" }
                                a { href: "https://discord.gg/kTzKSUcdZ6", target: "_blank", "Discord" }
                            }
                        }
                    }

                    div {
                        class: "contact-form-container",
                        if submitted() {
                            div {
                                class: "success-message",
                                h3 { "Thanks for reaching out!" }
                                p { "I'll get back to you as soon as possible." }
                            }
                        } else {
                            form {
                                class: "contact-form",
                                onsubmit: handle_submit,
                                prevent_default: "onsubmit",
                                
                                div {
                                    class: "form-group",
                                    label { r#for: "name", "Name" }
                                    input {
                                        r#type: "text",
                                        id: "name",
                                        required: true,
                                        value: "{name}",
                                        oninput: move |e| name.set(e.value()),
                                    }
                                }
                                
                                div {
                                    class: "form-group",
                                    label { r#for: "email", "Email" }
                                    input {
                                        r#type: "email",
                                        id: "email",
                                        required: true,
                                        value: "{email}",
                                        oninput: move |e| email.set(e.value()),
                                    }
                                }
                                
                                div {
                                    class: "form-group",
                                    label { r#for: "message", "Message" }
                                    textarea {
                                        id: "message",
                                        rows: "5",
                                        required: true,
                                        value: "{message}",
                                        oninput: move |e| message.set(e.value()),
                                    }
                                }
                                
                                button {
                                    r#type: "submit",
                                    class: "btn btn-primary submit-btn",
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