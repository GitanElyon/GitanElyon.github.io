use scraper::{ElementRef, Html, Selector};

const RESUME_URL: &str = "https://gitanelyon.dev/resume/";

#[derive(Clone, PartialEq)]
pub struct ResumeData {
    pub name: String,
    pub subtitle: String,
    pub title: String,
    pub summary: String,
    pub education: Vec<ResumeEducation>,
    pub skills: Vec<ResumeSkillCategory>,
    pub contact: Vec<ResumeContactItem>,
    pub experience: Vec<ResumeEntry>,
    pub projects: Vec<ResumeEntry>,
}

#[derive(Clone, PartialEq)]
pub struct ResumeEducation {
    pub institution: String,
    pub detail: String,
}

#[derive(Clone, PartialEq)]
pub struct ResumeSkillCategory {
    pub title: String,
    pub skills: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct ResumeContactItem {
    pub label: String,
    pub value: String,
    pub href: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct ResumeEntry {
    pub title: String,
    pub meta: String,
    pub bullets: Vec<String>,
}

pub async fn fetch_resume_data() -> Result<ResumeData, String> {
    let html = reqwest::get(RESUME_URL)
        .await
        .map_err(|err| format!("Failed to fetch resume: {err}"))?
        .text()
        .await
        .map_err(|err| format!("Failed to read resume HTML: {err}"))?;

    Ok(parse_resume_html(&html))
}

fn parse_resume_html(html: &str) -> ResumeData {
    let document = Html::parse_document(html);

    let sidebar_header = Selector::parse(".sidebar-header").unwrap();
    let sidebar_section = Selector::parse(".sidebar-section").unwrap();
    let education_item = Selector::parse(".education-item").unwrap();
    let skill_category = Selector::parse(".skill-category").unwrap();
    let contact_item = Selector::parse(".contact-item").unwrap();
    let main_content = Selector::parse(".main-content .content-section").unwrap();
    let job = Selector::parse(".job").unwrap();

    let header = document.select(&sidebar_header).next();

    let name = header
        .and_then(|el| text_for_selector(el, "h1"))
        .unwrap_or_else(|| "Gitan Elyon".to_string());
    let subtitle = header
        .and_then(|el| text_for_selector(el, "h2"))
        .unwrap_or_else(|| "Mandell-Balogh".to_string());
    let title = header
        .and_then(|el| text_for_selector(el, ".title"))
        .unwrap_or_else(|| "Software Engineer".to_string());

    let education = document
        .select(&sidebar_section)
        .find(|section| heading_text(*section) == Some("Education".to_string()))
        .and_then(|section| section.select(&education_item).next())
        .map(|item| ResumeEducation {
            institution: text_for_selector(item, ".edu-institution")
                .or_else(|| text_for_selector(item, "p:first-child"))
                .unwrap_or_default(),
            detail: text_for_selector(item, ".edu-detail")
                .or_else(|| text_for_selector(item, "p:last-child"))
                .unwrap_or_default(),
        })
        .into_iter()
        .collect();

    let skills = document
        .select(&sidebar_section)
        .find(|section| heading_text(*section) == Some("Skills".to_string()))
        .map(|section| {
            section
                .select(&skill_category)
                .map(|category| ResumeSkillCategory {
                    title: text_for_selector(category, "h3").unwrap_or_default(),
                    skills: category
                        .select(&Selector::parse("li").unwrap())
                        .map(|skill| normalized_text(&skill.text().collect::<Vec<_>>().join(" ")))
                        .filter(|skill| !skill.is_empty())
                        .collect(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let contact = document
        .select(&sidebar_section)
        .find(|section| heading_text(*section) == Some("Contact".to_string()))
        .map(|section| {
            section
                .select(&contact_item)
                .map(|item| {
                    let label = text_for_selector(item, ".contact-label")
                        .or_else(|| text_for_selector(item, "p:first-child"))
                        .unwrap_or_default();
                    let value = item
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .map(|a| normalized_text(&a.text().collect::<Vec<_>>().join(" ")))
                        .or_else(|| text_for_selector(item, "p:last-child"))
                        .unwrap_or_default();
                    let href = item
                        .select(&Selector::parse("a").unwrap())
                        .next()
                        .and_then(|a| a.value().attr("href"))
                        .map(|s| s.to_string());

                    ResumeContactItem { label, value, href }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let summary = document
        .select(&main_content)
        .find(|section| heading_text(*section) == Some("Summary".to_string()))
        .and_then(|section| text_for_selector(section, "p"))
        .unwrap_or_default();

    let experience = document
        .select(&main_content)
        .find(|section| heading_text(*section) == Some("Professional Experience".to_string()))
        .map(|section| section_to_entries(section, &job))
        .unwrap_or_default();

    let projects = document
        .select(&main_content)
        .find(|section| heading_text(*section) == Some("Technical Projects".to_string()))
        .map(|section| section_to_entries(section, &job))
        .unwrap_or_default();

    ResumeData {
        name,
        subtitle,
        title,
        summary,
        education,
        skills,
        contact,
        experience,
        projects,
    }
}

fn section_to_entries(section: ElementRef<'_>, job_selector: &Selector) -> Vec<ResumeEntry> {
    section
        .select(job_selector)
        .map(|job| ResumeEntry {
            title: text_for_selector(job, "h3").unwrap_or_default(),
            meta: text_for_selector(job, ".job-meta")
                .or_else(|| text_for_selector(job, "p"))
                .unwrap_or_default(),
            bullets: job
                .select(&Selector::parse("li").unwrap())
                .map(|bullet| normalized_text(&bullet.text().collect::<Vec<_>>().join(" ")))
                .filter(|bullet| !bullet.is_empty())
                .collect(),
        })
        .collect()
}

fn heading_text(section: ElementRef<'_>) -> Option<String> {
    text_for_selector(section, "h2").or_else(|| text_for_selector(section, "h3"))
}

fn text_for_selector(element: ElementRef<'_>, selector: &str) -> Option<String> {
    Selector::parse(selector)
        .ok()
        .and_then(|selector| element.select(&selector).next())
        .map(|node| normalized_text(&node.text().collect::<Vec<_>>().join(" ")))
}

fn normalized_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ").trim().to_string()
}