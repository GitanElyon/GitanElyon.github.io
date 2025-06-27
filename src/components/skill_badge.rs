use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SkillBadgeProps {
    name: String,
    level: String, // "Beginner", "Intermediate", "Advanced", "Expert"
}

#[component]
pub fn SkillBadge(props: SkillBadgeProps) -> Element {
    let level_class = match props.level.as_str() {
        "Expert" => "skill-expert",
        "Advanced" => "skill-advanced", 
        "Intermediate" => "skill-intermediate",
        _ => "skill-beginner",
    };

    rsx! {
        div {
            class: "skill-badge {level_class}",
            span { class: "skill-name", "{props.name}" }
            span { class: "skill-level", "{props.level}" }
        }
    }
}