use dioxus::prelude::*;

// Define icon assets
const RUST_ICON: Asset = asset!("/icons/langs/rust.svg");
const GO_ICON: Asset = asset!("/icons/langs/go.svg");
const PYTHON_ICON: Asset = asset!("/icons/langs/python.svg");
const JS_ICON: Asset = asset!("/icons/langs/js.svg");
const _TS_ICON: Asset = asset!("/icons/langs/ts.svg");
const JAVA_ICON: Asset = asset!("/icons/langs/java.svg");
const HTML_ICON: Asset = asset!("/icons/langs/html-5.svg");
const _CSS_ICON: Asset = asset!("/icons/langs/css-3.svg");
const C_ICON: Asset = asset!("/icons/langs/c.svg");
const _CPP_ICON: Asset = asset!("/icons/langs/cpp.svg");
const CSHARP_ICON: Asset = asset!("/icons/langs/csharp.svg");
const LUA_ICON: Asset = asset!("/icons/langs/lua.svg");

const TAURI_ICON: Asset = asset!("/icons/frameworks/tauri.svg");
const BUN_ICON: Asset = asset!("/icons/frameworks/bun.svg");
const DIOXUS_ICON: Asset = asset!("/icons/frameworks/dioxus.png");
const NODE_ICON: Asset = asset!("/icons/frameworks/node-js.svg");
const SVELTE_ICON: Asset = asset!("/icons/frameworks/svelte.svg");
const VUE_ICON: Asset = asset!("/icons/frameworks/vue.svg");
const REACT_ICON: Asset = asset!("/icons/frameworks/react.svg");
const ELECTRON_ICON: Asset = asset!("/icons/frameworks/electron.svg");

const LINUX_ICON: Asset = asset!("/icons/tools/linux.svg");
const GIT_ICON: Asset = asset!("/icons/tools/git.svg");
const NIX_ICON: Asset = asset!("/icons/tools/nix.svg");
const DOCKER_ICON: Asset = asset!("/icons/tools/docker.svg");
const ADOBE_ICON: Asset = asset!("/icons/tools/adobe.svg");
const FIGMA_ICON: Asset = asset!("/icons/tools/figma.svg");

const POSTGRES_ICON: Asset = asset!("/icons/data/postgresql.svg");
const SOLR_ICON: Asset = asset!("/icons/data/apachesolr.svg");
const SURREALDB_ICON: Asset = asset!("/icons/data/surrealdb.svg");
const CLOUDFLARE_ICON: Asset = asset!("/icons/data/cloudflare.svg");

#[derive(Clone, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub icon: Asset,
    pub hours: u32,
    pub projects: Vec<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct LanguageAbout {
    pub name: &'static str,
    pub brief: &'static str,
    pub long: &'static str,
    pub years: &'static str,
    pub tools: Vec<&'static str>,
    pub color: &'static str,
}

pub fn get_skill_sections() -> Vec<(&'static str, Vec<Skill>)> {
    vec![
        (
            "Languages",
            vec![
                Skill { name: "Rust", icon: RUST_ICON, hours: 300, projects: vec!["Minos", "Portfolio", "Blackjack AI", "Suilend Liquidator", "qst"] },
                Skill { name: "Go", icon: GO_ICON, hours: 100, projects: vec!["Backend APIs", "Microservices"] },
                Skill { name: "Python", icon: PYTHON_ICON, hours: 150, projects: vec!["ML Models", "Data Analysis", "Automation"] },
                Skill { name: "JS/TS", icon: JS_ICON, hours: 250, projects: vec!["Web Apps", "Node Services", "Full Stack Apps", "Type-safe APIs"] },
                Skill { name: "Java", icon: JAVA_ICON, hours: 80, projects: vec!["Enterprise Apps", "Android"] },
                Skill { name: "HTML/CSS", icon: HTML_ICON, hours: 220, projects: vec!["Web Development", "UI/UX Design"] },
                Skill { name: "C/C++", icon: C_ICON, hours: 70, projects: vec!["Systems Programming", "Game Development"] },
                Skill { name: "C#", icon: CSHARP_ICON, hours: 60, projects: vec![".NET Apps"] },
                Skill { name: "Lua", icon: LUA_ICON, hours: 30, projects: vec!["Scripting", "Neovim Config"] },
            ],
        ),
        (
            "Frameworks",
            vec![
                Skill { name: "Tauri", icon: TAURI_ICON, hours: 200, projects: vec!["Desktop Apps"] },
                Skill { name: "Bun.js", icon: BUN_ICON, hours: 120, projects: vec!["Fast Runtimes"] },
                Skill { name: "Dioxus", icon: DIOXUS_ICON, hours: 150, projects: vec!["This Portfolio!"] },
                Skill { name: "Node.js", icon: NODE_ICON, hours: 40, projects: vec!["Backend Services"] },
                Skill { name: "Svelte", icon: SVELTE_ICON, hours: 160, projects: vec!["Interactive UIs"] },
                Skill { name: "Vue", icon: VUE_ICON, hours: 80, projects: vec!["Papyr"] },
                Skill { name: "React", icon: REACT_ICON, hours: 20, projects: vec!["Web Apps"] },
                Skill { name: "Electron", icon: ELECTRON_ICON, hours: 40, projects: vec!["Cross-Platform Apps"] },
            ],
        ),
        (
            "Tools",
            vec![
                Skill { name: "Linux", icon: LINUX_ICON, hours: 1500, projects: vec!["Daily Driver", "Server Management"] },
                Skill { name: "Git", icon: GIT_ICON, hours: 800, projects: vec!["Version Control", "Collaboration"] },
                Skill { name: "Nix", icon: NIX_ICON, hours: 700, projects: vec!["Reproducible Builds"] },
                Skill { name: "Docker", icon: DOCKER_ICON, hours: 100, projects: vec!["Containerization"] },
                Skill { name: "Adobe", icon: ADOBE_ICON, hours: 200, projects: vec!["Design Work"] },
                Skill { name: "Figma", icon: FIGMA_ICON, hours: 150, projects: vec!["UI Design"] },
            ],
        ),
        (
            "Databases",
            vec![
                Skill { name: "PostgreSQL", icon: POSTGRES_ICON, hours: 50, projects: vec!["Relational DBs"] },
                Skill { name: "Solr", icon: SOLR_ICON, hours: 30, projects: vec!["Search Engines"] },
                Skill { name: "SurrealDB", icon: SURREALDB_ICON, hours: 20, projects: vec!["Modern DBs"] },
                Skill { name: "Cloudflare", icon: CLOUDFLARE_ICON, hours: 30, projects: vec!["Arcill"] },
            ],
        ),
    ]
}

pub fn get_about_languages() -> Vec<LanguageAbout> {
    vec![
        LanguageAbout {
            name: "Rust",
            brief: "Memory-safe and performant",
            long: "Rust is my go-to language for building memory-safe, high-performance applications. Rust often becomes the core of my projects thanks to its reliability and speed.",
            years: "~2 years",
            tools: vec!["Tauri", "Dioxus", "Suilend"],
            color: "#DEA584",
        },
        LanguageAbout {
            name: "Go",
            brief: "Simple, fast and reliable",
            long: "Go is my choice for building scalable backend services with simplicity and efficiency.",
            years: "~1.5 year",
            tools: vec!["Microservices", "REST", "gRPC"],
            color: "#00ADD8",
        },
        LanguageAbout {
            name: "Python",
            brief: "Versatile and powerful",
            long: "Python is my tool for prototyping, scripting, and automating anything I need done quickly.",
            years: "~5 years",
            tools: vec!["Pandas", "FastAPI", "NumPy"],
            color: "#3776AB",
        },
        LanguageAbout {
            name: "JavaScript",
            brief: "Dynamic and flexible",
            long: "JavaScript isn't my favorite stack to write in, but if I can dream it, JS can make it happen.",
            years: "~3.5 years",
            tools: vec!["Node", "Vue", "Svelte"],
            color: "#F7DF1E",
        },
    ]
}
