use dioxus::prelude::*;

#[component] 
pub fn Footer() -> Element {

    rsx! {
        footer {
            class: "max-w-[1200px] mx-auto py-12 md:py-16 text-center",

            div {
                class: "px-5 md:px-8",

                p {
                    class: "text-[#6b7280] text-sm",
                    "Built with ❤️ and Dioxus"
                }
            }
        }
    }
}