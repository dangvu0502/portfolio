use dioxus::prelude::*;

#[component] 
pub fn Footer() -> Element {

    rsx! {
        footer {
            class: "py-12 md:py-16 text-center",

            div {
                class: "max-w-[1200px] mx-auto px-5 md:px-8",

                p {
                    class: "text-[#6b7280] text-sm",
                    "Built with ❤️ and Dioxus"
                }
            }
        }
    }
}