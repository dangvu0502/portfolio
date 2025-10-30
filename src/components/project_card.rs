use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub thumbnail: String,
    pub tech_stack: Vec<String>,
    pub live_url: Option<String>,
    pub github_url: Option<String>,
}

#[component]
pub fn ProjectCard(project: Project) -> Element {
    rsx! {
        article {
            class: "bg-[#24272f] border border-[#2d3139] rounded-xl overflow-hidden transition-all duration-250 hover:-translate-y-1 hover:border-blue-500 hover:shadow-2xl shadow-lg group",

            // Project Image
            div {
                class: "w-full h-[200px] bg-gray-800 overflow-hidden",
                img {
                    src: "{project.thumbnail}",
                    alt: "Screenshot of {project.title}",
                    class: "w-full h-full object-cover",
                    loading: "lazy",
                }
            }

            // Content Area
            div {
                class: "p-6",

                // Title
                h3 {
                    class: "text-xl font-semibold text-white mb-3",
                    "{project.title}"
                }

                // Description
                p {
                    class: "text-gray-400 mb-4 leading-relaxed line-clamp-3",
                    "{project.description}"
                }

                // Tech Stack Badges
                div {
                    class: "flex flex-wrap gap-2 mb-4",
                    for tech in project.tech_stack.iter() {
                        span {
                            class: "px-3 py-1 text-xs bg-[#1a1d24] border border-[#404552] rounded text-gray-400",
                            "{tech}"
                        }
                    }
                }

                // Action Buttons
                div {
                    class: "flex gap-3",

                    if let Some(live_url) = &project.live_url {
                        a {
                            href: "{live_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm font-semibold rounded transition-colors duration-200 flex items-center gap-2",
                            aria_label: "View live demo of {project.title}",

                            // External link icon
                            svg {
                                class: "w-4 h-4",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                                }
                            }
                            "Live Demo"
                        }
                    }

                    if let Some(github_url) = &project.github_url {
                        a {
                            href: "{github_url}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "px-4 py-2 bg-transparent border border-[#404552] hover:border-blue-500 text-gray-400 hover:text-white text-sm font-semibold rounded transition-all duration-200 flex items-center gap-2",
                            aria_label: "View GitHub repository for {project.title}",

                            // GitHub icon
                            svg {
                                class: "w-4 h-4",
                                fill: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                                }
                            }
                            "GitHub"
                        }
                    }
                }
            }
        }
    }
}