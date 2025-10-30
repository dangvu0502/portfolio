use dioxus::prelude::*;
use crate::document::eval;

/// Fixed navigation bar with smooth scroll behavior and active section detection
#[component]
pub fn Navbar() -> Element {
    let mut active_section = use_signal(|| "home".to_string());

    // Smooth scroll function
    let mut scroll_to = move |section: &str| {
        let section = section.to_string();
        active_section.set(section.clone());

        eval(&format!(
            r#"
            const element = document.getElementById('{}');
            if (element) {{
                const navbarHeight = 64;
                const elementPosition = element.getBoundingClientRect().top;
                const offsetPosition = elementPosition + window.pageYOffset - navbarHeight;

                window.scrollTo({{
                    top: offsetPosition,
                    behavior: 'smooth'
                }});
            }}
            "#,
            section
        ));
    };

    rsx! {
        nav {
            class: "fixed top-0 left-0 right-0 z-50 bg-[rgba(15,17,22,0.8)] backdrop-blur-md border-b border-[#2d3139]",

            div {
                class: "max-w-[1280px] mx-auto px-6 md:px-12 h-14 md:h-16 flex items-center justify-between",

                // Logo/Brand
                button {
                    class: "text-white font-bold text-xl md:text-2xl transition-transform duration-200 hover:scale-105 cursor-pointer",
                    onclick: move |_| scroll_to("home"),
                    "Matt B"
                }

                // Navigation Links
                div {
                    class: "flex items-center gap-4 md:gap-8",

                    button {
                        class: if active_section() == "home" {
                            "text-[#3b82f6] font-medium transition-colors duration-200 border-b-2 border-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        } else {
                            "text-[#b4b8c5] font-medium transition-colors duration-200 hover:text-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        },
                        onclick: move |_| scroll_to("home"),
                        "Home"
                    }

                    button {
                        class: if active_section() == "projects" {
                            "text-[#3b82f6] font-medium transition-colors duration-200 border-b-2 border-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        } else {
                            "text-[#b4b8c5] font-medium transition-colors duration-200 hover:text-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        },
                        onclick: move |_| scroll_to("projects"),
                        "Projects"
                    }

                    button {
                        class: if active_section() == "contact" {
                            "text-[#3b82f6] font-medium transition-colors duration-200 border-b-2 border-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        } else {
                            "text-[#b4b8c5] font-medium transition-colors duration-200 hover:text-[#3b82f6] focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded px-2 py-1"
                        },
                        onclick: move |_| scroll_to("contact"),
                        "Contact"
                    }

                    // GitHub Link
                    a {
                        class: "text-[#b4b8c5] transition-all duration-200 hover:text-[#3b82f6] hover:scale-110 focus:outline-none focus:ring-2 focus:ring-[#3b82f6] focus:ring-offset-2 focus:ring-offset-[#0f1116] rounded p-1",
                        href: "https://github.com/yourusername",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        aria_label: "GitHub Profile",

                        // GitHub SVG Icon
                        svg {
                            class: "w-6 h-6",
                            fill: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            }
                        }
                    }
                }
            }
        }
    }
}
