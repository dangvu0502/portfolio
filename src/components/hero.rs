use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdLinkedin, LdGithub, LdMail};
use dioxus_free_icons::Icon;


#[component]
pub fn Hero() -> Element {
    rsx! {
        header {
            class: "max-w-[1200px] mx-auto  py-12 md:py-16 border-b border-[#1f1f1f]",

            div {
                class: "px-5 md:px-8",

                // Avatar and Social Icons
                div {
                    class: "flex flex-col md:flex-row justify-between items-center gap-6 md:gap-5 mb-10 md:mb-12",

                    // Avatar
                    img {
                        src: "https://avatars.githubusercontent.com/u/57172125",
                        alt: "Matt's avatar",
                        class: "w-20 h-20 rounded-full border-2 border-[#c46846] object-cover"
                    }

                    // Social Icons
                    div {
                        class: "flex gap-3 md:gap-4",

                        a {
                            href: "https://github.com/dangvu0502",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: "GitHub",
                            class: "w-9 h-9 bg-[#1a1a1a] border border-[#2a2a2a] rounded-md flex items-center justify-center hover:bg-[#2a2a2a] hover:border-[#2a2a2a] hover:-translate-y-0.5 transition-all duration-300",
                            Icon {
                                width: 30,
                                height: 30,
                                fill: "#e8dfd6",
                                icon: LdGithub,
                            }
                        }

                        a {
                            href: "mailto:dangvu0502@gmail.com",
                            aria_label: "Email",
                            class: "w-9 h-9 bg-[#1a1a1a] border border-[#2a2a2a] rounded-md flex items-center justify-center hover:bg-[#2a2a2a] hover:border-[#2a2a2a] hover:-translate-y-0.5 transition-all duration-300",
                            Icon {
                                width: 30,
                                height: 30,
                                fill: "#e8dfd6",
                                icon: LdMail,
                            }
                        }

                        a {
                            href: "https://www.linkedin.com/in/%C4%91%C4%83ng-v%C5%A9-a43920243/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: "LinkedIn",
                            class: "w-9 h-9 bg-[#1a1a1a] border border-[#2a2a2a] rounded-md flex items-center justify-center hover:bg-[#2a2a2a] hover:border-[#2a2a2a] hover:-translate-y-0.5 transition-all duration-300",
                            Icon {
                                width: 30,
                                height: 30,
                                fill: "#e8dfd6",
                                icon: LdLinkedin,
                            }
                        }

                    }
                }

                // Bio Section
                div {
                    class: "mb-8",

                    p {
                        class: "mb-4 text-base leading-relaxed",
                        dangerous_inner_html: "I'm <strong>Dang Vu (Matt)</strong>, a fullstack developer based in Vietnam."
                    }

                    p {
                        class: "mb-4 text-base leading-relaxed",
                        "Building performant, AI-powered web experiences with modern frameworks."
                    }

                    p {
                        class: "mb-4 text-base leading-relaxed",
                        "Currently exploring Rust and WebAssembly."
                    }

                    a {
                        href: "mailto:dangvu0502@gmail.com",
                        class: "inline-block mt-2.5 px-4 py-2 bg-[#c46846] text-white rounded-md font-medium hover:bg-[#b85a38] transition-colors duration-300",
                        "Hire me 🚀"
                    }
                }
            }
        }
    }
}
