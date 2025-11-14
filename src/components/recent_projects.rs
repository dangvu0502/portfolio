use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Props, Deserialize)]
pub struct RecentProject {
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub link: String,
}

impl RecentProject {
    pub fn from_json() -> Vec<RecentProject> {
        const RECENT_PROJECTS_DATA: &str = include_str!("../../data/recent-projects.json");
        serde_json::from_str(RECENT_PROJECTS_DATA).unwrap_or_else(|e| {
            eprintln!("Failed to parse recent projects: {}", e);
            vec![]
        })
    }
}

#[component]
pub fn RecentProjects(recent_projects: Vec<RecentProject>) -> Element {
    rsx! {
        section {
            id: "projects",
            class: "py-12 md:py-20 border-b border-[#1f1f1f]",

            div {
                class: "max-w-[1200px] mx-auto px-5 md:px-8",

                // Section Title
                h2 {
                    class: "text-2xl md:text-3xl font-bold text-white mb-10",
                    "Recent Projects"
                }

                // Projects Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 md:gap-6",

                    for recent_project in recent_projects {
                        RecentProjectCard {
                            recent_project: recent_project
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RecentProjectCard(recent_project: RecentProject) -> Element {
    rsx! {
    a {
        href: "{recent_project.link}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-6 hover:border-[#2a2a2a] hover:-translate-y-1 transition-all duration-300 block focus:outline-none focus:ring-2 focus:ring-[#c46846] focus:ring-offset-2 focus:ring-offset-[#0a0a0a]",

            h3 {
                class: "text-[#c46846] text-lg mb-3 font-semibold",
                "{recent_project.title}"
            }

            div {
                class: "flex flex-wrap gap-2 mb-3",
                for tag in recent_project.tags.iter() {
                    span {
                        class: "bg-[#1a1a1a] border border-[#2a2a2a] px-2.5 py-1 rounded text-xs text-[#9ca3af]",
                        "{tag}"
                    }
                }
            }

            if let Some(desc) = &recent_project.description {
                p {
                    class: "text-[#b0b0b0] text-sm leading-relaxed",
                    "{desc}"
                }
            }
        }
    }
}
