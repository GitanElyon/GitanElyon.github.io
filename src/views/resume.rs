use crate::data::resume::{fetch_resume_data, ResumeData, ResumeEducation, ResumeEntry, ResumeSkillCategory};
use dioxus::prelude::*;

fn trigger_print() {
    #[cfg(target_arch = "wasm32")]
    {
        let script = r#"
            (function() {
                const btn = document.activeElement;
                const oldText = btn && btn.tagName === 'BUTTON' ? btn.innerText : null;
                if(oldText) btn.innerText = "Preparing PDF...";
                
                fetch('https://gitanelyon.dev/resume/')
                    .then(res => res.text())
                    .then(html => {
                        const iframe = document.createElement('iframe');
                        iframe.style.position = 'fixed';
                        iframe.style.right = '0';
                        iframe.style.bottom = '0';
                        iframe.style.width = '816px';
                        iframe.style.height = '1056px';
                        iframe.style.opacity = '0';
                        iframe.style.pointerEvents = 'none';
                        document.body.appendChild(iframe);
                        
                        let doc = iframe.contentWindow.document;
                        let newHtml = html.replace('<head>', '<head><base href="https://gitanelyon.dev/resume/">');
                        
                        newHtml = newHtml.replace('</body>', `
                            <script>
                                window.addEventListener('load', () => {
                                    setTimeout(() => {
                                        const b = document.querySelector(".download-btn");
                                        if (b) b.click();
                                    }, 800);
                                });
                            </script>
                        </body>`);
                        
                        doc.open();
                        doc.write(newHtml);
                        doc.close();
                        
                        setTimeout(() => {
                            if(oldText) btn.innerText = oldText;
                            setTimeout(() => iframe.remove(), 10000);
                        }, 2500);
                    })
                    .catch(e => {
                        console.error('PDF fetch error:', e);
                        window.open('https://gitanelyon.dev/resume/', '_blank');
                        if(oldText) btn.innerText = oldText;
                    });
            })();
        "#;
        let _ = js_sys::eval(script);
    }
}

fn render_bullets(entry: &ResumeEntry) -> Element {
    rsx! {
        ul {
            for bullet in &entry.bullets {
                li { "{bullet}" }
            }
        }
    }
}

fn render_skill_category(category: &ResumeSkillCategory) -> Element {
    rsx! {
        div { class: "skill-category",
            h4 { "{category.title}" }
            p { "{category.skills.join(\", \")}" }
        }
    }
}

fn render_education_item(item: &ResumeEducation) -> Element {
    rsx! {
        div { class: "resume-item",
            h3 { "{item.institution}" }
            p { class: "company", "{item.detail}" }
        }
    }
}

fn render_entry(entry: &ResumeEntry) -> Element {
    rsx! {
        div { class: "resume-item",
            h3 { "{entry.title}" }
            p { class: "meta", "{entry.meta}" }
            {render_bullets(entry)}
        }
    }
}

#[component]
pub fn Resume() -> Element {
    let mut resume = use_signal(|| None::<Result<ResumeData, String>>);

    use_future(move || async move {
        let result = fetch_resume_data().await;
        resume.set(Some(result));
    });

    let on_download = move |_| trigger_print();

    rsx! {
        section { id: "resume",
            div { class: "container",
                h1 { class: "page-title", "Resume" }
                match resume().as_ref() {
                    Some(Ok(data)) => rsx! {
                        div { class: "resume-content",
                            div { class: "resume-section resume-header",
                                h1 { "{data.name}" }
                                h2 { "{data.subtitle}" }
                                p { class: "resume-title", "{data.title}" }
                            }

                            div { class: "resume-section",
                                h2 { "Summary" }
                                p { "{data.summary}" }
                            }

                            div { class: "resume-section",
                                h2 { "Professional Experience" }
                                for entry in &data.experience {
                                    {render_entry(entry)}
                                }
                            }

                            div { class: "resume-section",
                                h2 { "Technical Projects" }
                                for entry in &data.projects {
                                    {render_entry(entry)}
                                }
                            }

                            div { class: "resume-section",
                                h2 { "Education" }
                                for item in &data.education {
                                    {render_education_item(item)}
                                }
                            }

                            div { class: "resume-section",
                                h2 { "Technical Skills" }
                                div { class: "skills-categories",
                                    for category in &data.skills {
                                        {render_skill_category(category)}
                                    }
                                }
                            }

                            div { class: "resume-section",
                                h2 { "Contact" }
                                div { class: "contact-grid",
                                    for item in &data.contact {
                                        div { class: "contact-method",
                                            h4 { "{item.label}" }
                                            if let Some(href) = &item.href {
                                                a {
                                                    href: "{href}",
                                                    target: if href.starts_with("http") { "_blank" } else { "_self" },
                                                    "{item.value}"
                                                }
                                            } else {
                                                p { "{item.value}" }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "resume-actions",
                                button { class: "btn btn-primary", onclick: on_download, "Download PDF Resume" }
                                a {
                                    href: "https://gitanelyon.dev/resume/",
                                    target: "_blank",
                                    class: "btn btn-secondary",
                                    "Professional Resume ↗"
                                }
                            }
                        }
                    },
                    Some(Err(err)) => rsx! {
                        div { class: "resume-content",
                            p { class: "resume-contact", "Unable to load the live resume: {err}" }
                            div { class: "resume-actions",
                                button { class: "btn btn-primary", onclick: on_download, "Download PDF Resume" }
                                a {
                                    href: "https://gitanelyon.dev/resume/",
                                    target: "_blank",
                                    class: "btn btn-secondary",
                                    "Professional Resume ↗"
                                }
                            }
                        }
                    },
                    None => rsx! {
                        div { class: "resume-content",
                            p { class: "resume-contact", "Loading live resume..." }
                        }
                    },
                }
            }
        }
    }
}
