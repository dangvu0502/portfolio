use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone, PartialEq, Props, Deserialize)]
pub struct OSSContribution {
    pub repository: String,
    pub pr_number: String,
    pub link: Option<String>,
    #[serde(skip)]
    #[props(default)]
    pub title: Option<String>,
    #[serde(skip)]
    #[props(default)]
    pub state: Option<String>,
    #[serde(skip)]
    #[props(default)]
    pub created_at: Option<String>,
    #[serde(skip)]
    #[props(default)]
    pub updated_at: Option<String>,
}

/// Load OSS contributions from the generated JSON file
pub fn load_oss_contributions() -> Vec<OSSContribution> {
    const OSS_DATA: &str = include_str!("../../data/oss-contributions.json");

    serde_json::from_str(OSS_DATA).unwrap_or_else(|e| {
        eprintln!("Failed to parse OSS contributions: {}", e);
        vec![]
    })
}

#[component]
pub fn OSSContributionsSection(contributions: Vec<OSSContribution>) -> Element {
    rsx! {
        section {
            id: "oss-contributions",
            class: "py-12 md:py-20 border-b border-[#1f1f1f]",

            div {
                class: "max-w-[1200px] mx-auto px-5 md:px-8",

                h2 {
                    class: "text-2xl md:text-3xl text-white font-bold mb-8 md:mb-10",
                    "OSS Contributions"
                }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 md:gap-5",

                    for contribution in contributions {
                        OSSContributionCard { contribution: contribution }
                    }
                }
            }
        }
    }
}

#[component]
fn OSSContributionCard(contribution: OSSContribution) -> Element {
    if let Some(link) = &contribution.link {
        rsx! {
            a {
                href: "{link}",
                target: "_blank",
                rel: "noopener noreferrer",
                class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-5 hover:border-[#2a2a2a] transition-colors duration-300 block",

                p {
                    class: "text-[#b0b0b0] text-sm",
                    "Pull Request #{contribution.pr_number} · {contribution.repository}"
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "bg-[#111111] border border-[#1f1f1f] rounded-xl p-5 hover:border-[#2a2a2a] transition-colors duration-300 block",

                p {
                    class: "text-[#b0b0b0] text-sm",
                    "Pull Request #{contribution.pr_number} · {contribution.repository}"
                }
            }
        }
    }
}
