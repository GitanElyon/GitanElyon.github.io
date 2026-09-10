use crate::data::links::{get_bio_links, LinkIcon, LinkTarget};
use crate::data::profile::get_profile_data;
use crate::Route;
use dioxus::prelude::*;

const AVATAR: Asset = asset!("/assets/profile.webp");

/// Inline icon tinted via `currentColor` (the shared accent, cycling with
/// the background spectrum). Brand marks use their real path data.
fn bio_icon(icon: &LinkIcon) -> Element {
    rsx! {
        match icon {
            LinkIcon::Github => rsx! {
                svg {
                    class: "link-btn-icon",
                    view_box: "0 0 48 48",
                    width: "22",
                    height: "22",
                    fill: "currentColor",
                    "aria-hidden": "true",
                    path { d: "M6.81348 34.235C9.24811 38.3138 13.0939 41.4526 17.6772 42.9784C18.6779 43.1614 19.0425 42.5438 19.0425 42.0134C19.0425 41.7938 19.0388 41.4058 19.0339 40.8866C19.0282 40.2852 19.0208 39.5079 19.0155 38.6124C13.4524 39.8206 12.2787 35.931 12.2787 35.931C11.3689 33.6215 10.0576 33.0064 10.0576 33.0064C8.2417 31.7651 10.1951 31.7896 10.1951 31.7896C12.2025 31.9321 13.2584 33.8511 13.2584 33.8511C15.0424 36.9071 17.94 36.0243 19.0794 35.5135C19.2611 34.2207 19.7767 33.3391 20.3489 32.8394C15.908 32.3348 11.2387 30.6183 11.2387 22.9545C11.2387 20.7715 12.0184 18.9863 13.2977 17.5879C13.0914 17.082 12.4051 15.0488 13.4929 12.2949C13.4929 12.2949 15.1725 11.7571 18.9934 14.3453C20.5883 13.9021 22.2998 13.6798 24.0003 13.6725C25.6983 13.6798 27.4099 13.9021 29.0072 14.3453C32.8256 11.7571 34.5016 12.2949 34.5016 12.2949C35.5931 15.0488 34.9067 17.082 34.7005 17.5879C35.9823 18.9863 36.757 20.7715 36.757 22.9545C36.757 30.638 32.0804 32.3286 27.6247 32.8234C28.343 33.441 28.9827 34.6614 28.9827 36.5277C28.9827 38.3152 28.9717 39.8722 28.9644 40.9035C28.9608 41.4143 28.9581 41.7962 28.9581 42.0134C28.9581 42.5487 29.3178 43.1712 30.3332 42.976C33.9844 41.7572 37.1671 39.5154 39.5403 36.5903C35.8734 41.1108 30.274 44 23.9997 44C16.6943 44 10.3038 40.0832 6.81348 34.235Z" }
                }
            },
            LinkIcon::Linkedin => rsx! {
                svg {
                    class: "link-btn-icon",
                    view_box: "0 0 256 256",
                    width: "22",
                    height: "22",
                    fill: "currentColor",
                    "aria-hidden": "true",
                    path { d: "M218.123 218.127h-37.931v-59.403c0-14.165-.253-32.4-19.728-32.4-19.756 0-22.779 15.434-22.779 31.369v60.43h-37.93V95.967h36.413v16.694h.51a39.907 39.907 0 0 1 35.928-19.733c38.445 0 45.533 25.288 45.533 58.186l-.016 67.013ZM56.955 79.27c-12.157.002-22.014-9.852-22.016-22.009-.002-12.157 9.851-22.014 22.008-22.016 12.157-.003 22.014 9.851 22.016 22.008A22.013 22.013 0 0 1 56.955 79.27m18.966 138.858H37.95V95.967h37.97v122.16ZM237.033.018H18.89C8.58-.098.125 8.161-.001 18.471v219.053c.122 10.315 8.576 18.582 18.89 18.474h218.144c10.336.128 18.823-8.139 18.966-18.474V18.454c-.147-10.33-8.635-18.588-18.966-18.453" }
                }
            },
            _ => rsx! {
                svg {
                    class: "link-btn-icon",
                    view_box: "0 0 24 24",
                    width: "22",
                    height: "22",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    "aria-hidden": "true",
                    match icon {
                        LinkIcon::Globe => rsx! {
                            circle { cx: "12", cy: "12", r: "10" }
                            line { x1: "2", y1: "12", x2: "22", y2: "12" }
                            path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                        },
                        LinkIcon::Mail => rsx! {
                            path { d: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" }
                            polyline { points: "22,6 12,13 2,6" }
                        },
                        LinkIcon::User => rsx! {
                            path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                            circle { cx: "12", cy: "7", r: "4" }
                        },
                        _ => rsx! {
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                            line { x1: "16", y1: "13", x2: "8", y2: "13" }
                            line { x1: "16", y1: "17", x2: "8", y2: "17" }
                        },
                    }
                }
            },
        }
    }
}

#[component]
pub fn Links() -> Element {
    let profile = get_profile_data();
    let links = get_bio_links();
    let email_href = format!("mailto:{}", profile.email);

    rsx! {
        document::Title { "Links — Gitan Elyon" }
        section { id: "links",
            div { class: "links-card glass",
                img {
                    class: "links-avatar",
                    src: AVATAR,
                    alt: "Portrait of {profile.full_name}",
                    width: "224",
                    height: "224",
                    decoding: "async",
                }
                h1 { "{profile.full_name}" }
                p { class: "links-tagline", "{profile.title} · {profile.location}" }
                div { class: "links-list",
                    for link in links {
                        {
                            let arrow = match &link.target {
                                LinkTarget::Profile => "→",
                                _ => "↗",
                            };
                            match &link.target {
                                LinkTarget::Profile => rsx! {
                                    Link {
                                        to: Route::Profile {},
                                        class: "link-btn",
                                        {bio_icon(&link.icon)}
                                        span { class: "link-btn-text",
                                            span { class: "link-btn-label", "{link.label}" }
                                            span { class: "link-btn-sub", "{link.sub}" }
                                        }
                                        span { class: "link-btn-arrow", "{arrow}" }
                                    }
                                },
                                LinkTarget::Email => rsx! {
                                    a {
                                        href: "{email_href}",
                                        class: "link-btn",
                                        {bio_icon(&link.icon)}
                                        span { class: "link-btn-text",
                                            span { class: "link-btn-label", "{link.label}" }
                                            span { class: "link-btn-sub", "{link.sub}" }
                                        }
                                        span { class: "link-btn-arrow", "{arrow}" }
                                    }
                                },
                                LinkTarget::External(url) => rsx! {
                                    a {
                                        href: "{url}",
                                        target: "_blank",
                                        rel: "noopener me",
                                        class: "link-btn",
                                        {bio_icon(&link.icon)}
                                        span { class: "link-btn-text",
                                            span { class: "link-btn-label", "{link.label}" }
                                            span { class: "link-btn-sub", "{link.sub}" }
                                        }
                                        span { class: "link-btn-arrow", "{arrow}" }
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
