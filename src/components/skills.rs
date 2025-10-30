use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct SkillCategory {
    category: String,
    skills: Vec<String>,
}

#[component]
pub fn Skills() -> Element {
    // Skills data organized by category
    let skill_categories = vec![
        SkillCategory {
            category: "Frontend".to_string(),
            skills: vec![
                "React".to_string(),
                "TypeScript".to_string(),
                "Next.js".to_string(),
                "Tailwind CSS".to_string(),
                "HTML/CSS".to_string(),
                "JavaScript".to_string(),
            ],
        },
        SkillCategory {
            category: "AI Integration".to_string(),
            skills: vec![
                "OpenAI API".to_string(),
                "Langchain".to_string(),
                "Vector Databases".to_string(),
                "RAG".to_string(),
                "Prompt Engineering".to_string(),
            ],
        },
        SkillCategory {
            category: "Currently Learning".to_string(),
            skills: vec![
                "Rust".to_string(),
                "Dioxus".to_string(),
                "WebAssembly".to_string(),
                "Systems Programming".to_string(),
            ],
        },
    ];

    rsx! {
        section {
            id: "skills",
            class: "py-24 px-6 bg-[#1a1d24]",
            aria_label: "Technical skills and expertise",

            div {
                class: "max-w-4xl mx-auto",

                // Section Title
                h2 {
                    class: "text-3xl md:text-4xl font-bold text-white text-center mb-12",
                    "Tech Stack"
                }

                // Skill Categories
                div {
                    class: "space-y-10",

                    for category in skill_categories.iter() {
                        div {
                            key: "{category.category}",

                            // Category Title
                            h3 {
                                class: "text-xl font-semibold text-white mb-5",
                                "{category.category}"
                            }

                            // Skills Badges
                            div {
                                class: "flex flex-wrap gap-4",

                                for skill in category.skills.iter() {
                                    SkillBadge {
                                        skill: skill.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillBadge(skill: String) -> Element {
    rsx! {
        span {
            class: "px-5 py-3 bg-[#24272f] border border-[#404552] rounded-lg text-gray-400 font-medium transition-all duration-150 hover:border-blue-500 hover:text-white hover:scale-105 cursor-default focus:outline-2 focus:outline-blue-500 focus:outline-offset-2",
            tabindex: "0",
            "{skill}"
        }
    }
}
