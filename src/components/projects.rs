use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Props, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub link: String,
}

impl Project {
    pub fn from_json() -> Vec<Project> {
        const PROJECTS_DATA: &str = include_str!("../../data/projects.json");
        serde_json::from_str(PROJECTS_DATA).unwrap_or_else(|e| {
            eprintln!("Failed to parse projects: {}", e);
            vec![]
        })
    }
}

#[component]
pub fn ProjectsSection(projects: Vec<Project>) -> Element {
    rsx! {
        section {
            id: "projects",
            class: "py-12 md:py-20 border-b border-[#1f1f1f]",

            div {
                class: "max-w-[1200px] mx-auto px-5 md:px-8",

                h2 {
                    class: "text-2xl md:text-3xl text-white font-bold mb-8 md:mb-10",
                    "Projects"
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-5",

                    for project in projects {
                        ProjectCard { project: project }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectCard(project: Project) -> Element {
    rsx! {
        a {
            href: "{project.link}",
            target: "_blank",
            rel: "noopener noreferrer",
            class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-5 hover:border-[#2a2a2a] transition-colors duration-300 block",

            h3 {
                class: "text-white text-base font-semibold mb-1",
                "{project.name}"
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
