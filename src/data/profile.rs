//! Profile data parsed at compile time from the vendored `assets/profile.toml`.
//!
//! Source of truth lives in https://github.com/GitanElyon/resume (`resume.toml`).
//! See `build.rs` (sibling-checkout auto-sync) and `scripts/sync-profile.sh`.

use serde::Deserialize;

const PROFILE_TOML: &str = include_str!("../../assets/profile.toml");

// ── Raw TOML shapes (mirror resume.toml) ─────────────────────────────────

#[derive(Debug, Deserialize, Default)]
struct ProfileToml {
    #[serde(default)]
    personal: Personal,
    #[serde(default)]
    contact: Contact,
    #[serde(default)]
    education: Education,
    #[serde(default)]
    summary: Summary,
    #[serde(default)]
    work: Work,
    #[serde(default)]
    projects: ProjectsSection,
    #[serde(default)]
    skills: Skills,
    #[serde(default)]
    other: Other,
}

#[derive(Debug, Deserialize, Default)]
struct Personal {
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    title: String,
}

#[derive(Debug, Deserialize, Default)]
struct Contact {
    #[serde(default)]
    website: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    linkedin: String,
    #[serde(default)]
    github: String,
    #[serde(default)]
    phone: String,
    #[serde(default)]
    location: String,
    #[serde(default)]
    citizenship: String,
}

#[derive(Debug, Deserialize, Default)]
struct Education {
    #[serde(default)]
    school: String,
    #[serde(default)]
    degree: String,
    #[serde(default)]
    major: String,
    #[serde(default)]
    graduation: String,
}

#[derive(Debug, Deserialize, Default)]
struct Summary {
    #[serde(default)]
    summary: String,
}

#[derive(Debug, Deserialize, Default)]
struct Work {
    #[serde(default)]
    jobs: Vec<Job>,
}

#[derive(Debug, Deserialize, Default)]
struct Job {
    #[serde(default)]
    company: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    start_date: String,
    #[serde(default)]
    end_date: String,
    #[serde(default)]
    bullets: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ProjectsSection {
    #[serde(default)]
    projects: Vec<TomlProject>,
}

#[derive(Debug, Deserialize, Default)]
struct TomlProject {
    #[serde(default)]
    name: String,
    #[serde(default)]
    skills: Vec<String>,
    #[serde(default)]
    date: String,
    #[serde(default)]
    github: String,
    #[serde(default)]
    bullets: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct Skills {
    #[serde(default)]
    languages: Vec<String>,
    #[serde(default)]
    frameworks: Vec<String>,
    #[serde(default)]
    data: Vec<String>,
    #[serde(default)]
    tools: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct Other {
    #[serde(default)]
    publications: Vec<Publication>,
}

#[derive(Debug, Deserialize, Default, Clone)]
struct Publication {
    #[serde(default)]
    title: String,
    #[serde(default)]
    role: String,
    #[serde(default)]
    publisher: String,
    #[serde(default)]
    date: String,
    #[serde(default)]
    link: String,
    #[serde(default)]
    description: String,
}

// ── UI-facing shapes ─────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileData {
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub title: String,
    pub website: String,
    pub email: String,
    pub linkedin: String,
    pub github: String,
    pub phone: String,
    pub location: String,
    pub citizenship: String,
    pub school: String,
    pub degree: String,
    pub major: String,
    pub graduation: String,
    pub summary: String,
    pub experience: Vec<ProfileEntry>,
    pub projects: Vec<ProfileEntry>,
    pub skills: Vec<ProfileSkillCategory>,
    pub publications: Vec<ProfilePublication>,
    pub contact_items: Vec<ProfileContactItem>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileEntry {
    pub title: String,
    pub meta: String,
    pub bullets: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileSkillCategory {
    pub title: String,
    pub skills: Vec<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProfilePublication {
    pub title: String,
    pub detail: String,
    pub description: String,
    pub link: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileContactItem {
    pub label: String,
    pub value: String,
    pub href: Option<String>,
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn month_name(month: &str) -> Option<&'static str> {
    match month {
        "01" => Some("January"),
        "02" => Some("February"),
        "03" => Some("March"),
        "04" => Some("April"),
        "05" => Some("May"),
        "06" => Some("June"),
        "07" => Some("July"),
        "08" => Some("August"),
        "09" => Some("September"),
        "10" => Some("October"),
        "11" => Some("November"),
        "12" => Some("December"),
        _ => None,
    }
}

/// "2026-04" -> "April 2026"; anything else passes through untouched.
pub fn format_date(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() == 7 && bytes[4] == b'-' {
        let year = &value[0..4];
        let month = &value[5..7];
        if year.bytes().all(|b| b.is_ascii_digit()) {
            if let Some(name) = month_name(month) {
                return format!("{name} {year}");
            }
        }
    }
    value.to_string()
}

fn degree_short(degree: &str) -> &str {
    match degree {
        "Bachelor of Science" => "B.S.",
        other => other,
    }
}

fn display_url(url: &str) -> String {
    let mut out = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url)
        .strip_prefix("www.")
        .unwrap_or(
            url.strip_prefix("https://")
                .or_else(|| url.strip_prefix("http://"))
                .unwrap_or(url),
        );
    // strip a single trailing slash for display only
    if out.ends_with('/') && out.len() > 1 {
        out = &out[..out.len() - 1];
    }
    // re-strip www. in case scheme-strip happened first
    out.strip_prefix("www.").unwrap_or(out).to_string()
}

fn fallback_data() -> ProfileData {
    ProfileData {
        first_name: "Gitan Elyon".into(),
        last_name: "Mandell-Balogh".into(),
        full_name: "Gitan Elyon Mandell-Balogh".into(),
        title: "Systems Engineer".into(),
        website: "https://gitanelyon.dev".into(),
        email: "gitanelyon@gmail.com".into(),
        linkedin: "https://www.linkedin.com/in/gitanelyon/".into(),
        github: "https://github.com/gitanelyon".into(),
        phone: "443-224-8540".into(),
        location: "Baltimore, MD".into(),
        citizenship: String::new(),
        school: "University of Maryland".into(),
        degree: "B.S.".into(),
        major: "Information Science".into(),
        graduation: "Expected May 2028".into(),
        summary: String::new(),
        experience: vec![],
        projects: vec![],
        skills: vec![],
        publications: vec![],
        contact_items: vec![],
    }
}

fn push_skill_group(out: &mut Vec<ProfileSkillCategory>, title: &str, skills: &[String]) {
    if !skills.is_empty() {
        out.push(ProfileSkillCategory {
            title: title.to_string(),
            skills: skills.to_vec(),
        });
    }
}

// ── Public API ───────────────────────────────────────────────────────────

/// Parse the bundled TOML. Fails only if the vendored file is corrupt;
/// callers should prefer [`get_profile_data`] which falls back gracefully.
pub fn parse_profile_toml() -> Result<ProfileData, String> {
    let raw: ProfileToml =
        toml::from_str(PROFILE_TOML).map_err(|e| format!("Failed to parse profile.toml: {e}"))?;

    let full_name = format!("{} {}", raw.personal.first_name, raw.personal.last_name)
        .trim()
        .to_string();

    let experience = raw
        .work
        .jobs
        .iter()
        .map(|job| ProfileEntry {
            title: format!("{} — {}", job.title, job.company),
            meta: format!(
                "{} | {} – {}",
                job.company,
                format_date(&job.start_date),
                format_date(&job.end_date)
            ),
            bullets: job.bullets.clone(),
        })
        .collect();

    let projects = raw
        .projects
        .projects
        .iter()
        .map(|project| {
            let meta = if project.skills.is_empty() {
                format_date(&project.date)
            } else {
                format!(
                    "{} | {}",
                    project.skills.join(", "),
                    format_date(&project.date)
                )
            };
            let mut bullets = project.bullets.clone();
            if project.github != "private" && !project.github.is_empty() {
                // Surface the repo link like build.sh does (appended to last bullet
                // is handled in the view via `github_url` instead; keep bullets clean).
                let _ = &mut bullets;
            }
            ProfileEntry {
                title: project.name.clone(),
                meta,
                bullets,
            }
        })
        .collect();

    let mut skills = Vec::with_capacity(4);
    push_skill_group(&mut skills, "Languages", &raw.skills.languages);
    push_skill_group(
        &mut skills,
        "Frameworks & Libraries",
        &raw.skills.frameworks,
    );
    push_skill_group(&mut skills, "Databases & Search", &raw.skills.data);
    push_skill_group(&mut skills, "Tools & Systems", &raw.skills.tools);

    let publications = raw
        .other
        .publications
        .iter()
        .map(|publication| ProfilePublication {
            title: publication.title.clone(),
            detail: format!(
                "{} — {}, {}",
                publication.role,
                publication.publisher,
                format_date(&publication.date)
            ),
            description: publication.description.clone(),
            link: if publication.link.is_empty() {
                None
            } else {
                Some(publication.link.clone())
            },
        })
        .collect();

    let website_display = display_url(&raw.contact.website);
    let github_display = display_url(&raw.contact.github);
    let linkedin_display = display_url(&raw.contact.linkedin);
    let _ = website_display;

    let contact_items = vec![
        ProfileContactItem {
            label: "Website".into(),
            value: display_url(&raw.contact.website),
            href: Some(raw.contact.website.clone()),
        },
        ProfileContactItem {
            label: "GitHub".into(),
            value: github_display,
            href: Some(raw.contact.github.clone()),
        },
        ProfileContactItem {
            label: "LinkedIn".into(),
            value: linkedin_display,
            href: Some(raw.contact.linkedin.clone()),
        },
        ProfileContactItem {
            label: "Email".into(),
            value: raw.contact.email.clone(),
            href: Some(format!("mailto:{}", raw.contact.email)),
        },
        ProfileContactItem {
            label: "Location".into(),
            value: raw.contact.location.clone(),
            href: None,
        },
    ];

    Ok(ProfileData {
        full_name,
        first_name: raw.personal.first_name,
        last_name: raw.personal.last_name,
        title: raw.personal.title,
        website: raw.contact.website,
        email: raw.contact.email,
        linkedin: raw.contact.linkedin,
        github: raw.contact.github,
        phone: raw.contact.phone,
        location: raw.contact.location,
        citizenship: raw.contact.citizenship,
        school: raw.education.school,
        degree: degree_short(&raw.education.degree).to_string(),
        major: raw.education.major,
        graduation: raw.education.graduation,
        summary: raw.summary.summary,
        experience,
        projects,
        skills,
        publications,
        contact_items,
    })
}

/// Compile-time-bundled profile. Never panics: falls back to a minimal
/// hardcoded profile if the TOML is somehow corrupt.
pub fn get_profile_data() -> ProfileData {
    parse_profile_toml().unwrap_or_else(|_| fallback_data())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_vendored_toml() {
        let data = parse_profile_toml().expect("vendored profile.toml must parse");
        assert_eq!(data.first_name, "Gitan Elyon");
        assert_eq!(data.last_name, "Mandell-Balogh");
        assert!(!data.experience.is_empty());
        assert!(!data.skills.is_empty());
        assert_eq!(data.experience[0].meta, "Ridgeback | April 2026 – Present");
    }

    #[test]
    fn formats_dates() {
        assert_eq!(format_date("2026-04"), "April 2026");
        assert_eq!(format_date("2025-10"), "October 2025");
        assert_eq!(format_date("Present"), "Present");
        assert_eq!(format_date(""), "");
    }

    #[test]
    fn shortens_degree() {
        assert_eq!(degree_short("Bachelor of Science"), "B.S.");
        assert_eq!(degree_short("PhD"), "PhD");
    }
}
