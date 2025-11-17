use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Props, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub link: String,
}

impl Project {
    pub fn from_pinned_projects_json() -> Vec<Project> {
        const PINNED_PROJECTS_DATA: &str = include_str!("../../data/pinned-projects.json");
        serde_json::from_str(PINNED_PROJECTS_DATA).unwrap_or_else(|e| {
            eprintln!("Failed to parse projects: {}", e);
            vec![]
        })
    }
}

#[component]
pub fn PinnedProjectsSection(projects: Vec<Project>) -> Element {
    rsx! {
        section {
            id: "pinned-projects",
            class: "max-w-[1200px] mx-auto py-12 md:py-20 border-b border-[#1f1f1f]",

            div {
                class: "px-5 md:px-8",

                // Section Title
                h2 {
                    class: "text-2xl md:text-3xl font-bold text-white mb-10",
                    "Pinned"
                }

                // Projects Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 md:gap-6",

                    for project in projects {
                        PinnedProjectCard {
                            project: project
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PinnedProjectCard(project: Project) -> Element {
    rsx! {
    a {
        href: "{project.link}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-6 hover:border-[#2a2a2a] hover:-translate-y-1 transition-all duration-300 block focus:outline-none focus:ring-2 focus:ring-[#c46846] focus:ring-offset-2 focus:ring-offset-[#0a0a0a]",

            h3 {
                class: "text-[#c46846] text-lg mb-3 font-semibold",
                "{project.name}"
            }

            div {
                class: "flex flex-wrap gap-2 mb-3",
                for topic in project.topics.iter() {
                    span {
                        class: "bg-[#1a1a1a] border border-[#2a2a2a] px-2.5 py-1 rounded text-xs text-[#9ca3af]",
                        "{topic}"
                    }
                }
            }

            if let Some(desc) = &project.description {
                p {
                    class: "text-[#b0b0b0] text-sm leading-relaxed",
                    "{desc}"
                }
            }
        }
    }
}

impl Project {
    pub fn from_recent_projects_json() -> Vec<Project> {
        const RECENT_PROJECTS_DATA: &str = include_str!("../../data/recent-projects.json");
        serde_json::from_str(RECENT_PROJECTS_DATA).unwrap_or_else(|e| {
            eprintln!("Failed to parse recent projects: {}", e);
            vec![]
        })
    }
}

#[component]
pub fn RecentProjectsSection(projects: Vec<Project>) -> Element {
    rsx! {
        section {
            id: "recent-projects",
            class: "max-w-[1200px] mx-auto py-12 md:py-20 border-b border-[#1f1f1f]",

            div {
                class: "px-5 md:px-8",

                h2 {
                    class: "text-2xl md:text-3xl text-white font-bold mb-8 md:mb-10",
                    "Recent Projects"
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-5",

                    for project in projects {
                        RecentProjectCard { project: project }
                    }
                }
            }
        }
    }
}

#[component]
fn RecentProjectCard(project: Project) -> Element {
    rsx! {
        a {
            href: "{project.link}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-5 hover:border-[#2a2a2a] transition-colors duration-300 block",

            h3 {
                class: "text-white text-base font-semibold mb-3",
                "{project.name}"
            }

            if project.topics.len() > 0 {
                div {
                    class: "flex flex-wrap gap-2 mb-3",
                    for topic in project.topics.iter() {
                        span {
                            class: "bg-[#1a1a1a] border border-[#2a2a2a] px-2.5 py-1 rounded text-xs text-[#9ca3af]",
                            "{topic}"
                        }
                    }
                }
            }

            if let Some(desc) = &project.description {
                p {
                    class: "text-[#b0b0b0] text-sm",
                    "{desc}"
                }
            }
        }
    }
}
