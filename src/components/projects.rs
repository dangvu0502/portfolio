use dioxus::prelude::*;
use crate::components::{Project, ProjectCard};

#[component]
pub fn Projects() -> Element {
    // Sample project data
    let projects = vec![
        Project {
            title: "AI Chat Assistant".to_string(),
            description: "Real-time chat interface with OpenAI integration, streaming responses, and conversation history management.".to_string(),
            thumbnail: "/assets/projects/ai-chat.jpg".to_string(),
            tech_stack: vec![
                "React".to_string(),
                "TypeScript".to_string(),
                "OpenAI API".to_string(),
                "Tailwind CSS".to_string(),
            ],
            live_url: Some("https://example.com/demo".to_string()),
            github_url: Some("https://github.com/username/ai-chat".to_string()),
        },
        Project {
            title: "E-Commerce Dashboard".to_string(),
            description: "Admin dashboard for managing products, orders, and analytics with real-time updates and data visualization.".to_string(),
            thumbnail: "/assets/projects/dashboard.jpg".to_string(),
            tech_stack: vec![
                "Next.js".to_string(),
                "TypeScript".to_string(),
                "Prisma".to_string(),
                "PostgreSQL".to_string(),
            ],
            live_url: Some("https://example.com/dashboard".to_string()),
            github_url: Some("https://github.com/username/dashboard".to_string()),
        },
        Project {
            title: "Portfolio Generator".to_string(),
            description: "Tool to generate beautiful developer portfolios from GitHub profiles using AI-powered content generation.".to_string(),
            thumbnail: "/assets/projects/portfolio-gen.jpg".to_string(),
            tech_stack: vec![
                "React".to_string(),
                "OpenAI".to_string(),
                "Tailwind".to_string(),
                "Vite".to_string(),
            ],
            live_url: Some("https://example.com/portfolio-gen".to_string()),
            github_url: Some("https://github.com/username/portfolio-gen".to_string()),
        },
    ];

    rsx! {
        section {
            id: "projects",
            class: "py-24 px-6 bg-[#0f1116]",

            div {
                class: "max-w-7xl mx-auto",

                // Section Title
                h2 {
                    class: "text-3xl md:text-4xl font-bold text-white text-center mb-12",
                    "Featured Projects"
                }

                // Projects Grid
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",

                    for project in projects {
                        ProjectCard {
                            project: project
                        }
                    }
                }
            }
        }
    }
}
