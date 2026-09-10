//! Link-in-bio entries for the standalone `/links` page.
//!
//! Icons are inline SVGs (see `views::links`) tinted with the shared accent
//! color, so they cycle with the background spectrum.

#[derive(Clone, PartialEq)]
pub enum LinkIcon {
    Globe,
    Github,
    Linkedin,
    Mail,
    User,
    Doc,
}

#[derive(Clone, PartialEq)]
pub enum LinkTarget {
    /// Absolute URL opened in a new tab.
    External(&'static str),
    /// `mailto:` address (resolved from profile data at render time).
    Email,
    /// In-app route rendered without a full page load.
    Profile,
}

#[derive(Clone, PartialEq)]
pub struct BioLink {
    pub label: &'static str,
    pub sub: &'static str,
    pub icon: LinkIcon,
    pub target: LinkTarget,
}

pub fn get_bio_links() -> Vec<BioLink> {
    vec![
        BioLink {
            label: "Website",
            sub: "gitanelyon.dev",
            icon: LinkIcon::Globe,
            target: LinkTarget::External("https://gitanelyon.dev"),
        },
        BioLink {
            label: "GitHub",
            sub: "@GitanElyon",
            icon: LinkIcon::Github,
            target: LinkTarget::External("https://github.com/GitanElyon"),
        },
        BioLink {
            label: "LinkedIn",
            sub: "Gitan Elyon",
            icon: LinkIcon::Linkedin,
            target: LinkTarget::External("https://www.linkedin.com/in/gitanelyon/"),
        },
        BioLink {
            label: "Email",
            sub: "gitanelyon@gmail.com",
            icon: LinkIcon::Mail,
            target: LinkTarget::Email,
        },
        BioLink {
            label: "Profile",
            sub: "Experience, projects & publications",
            icon: LinkIcon::User,
            target: LinkTarget::Profile,
        },
        BioLink {
            label: "Printable Resume",
            sub: "gitanelyon.dev/resume",
            icon: LinkIcon::Doc,
            target: LinkTarget::External("https://gitanelyon.dev/resume/"),
        },
    ]
}
