use sycamore::prelude::*;

#[component(Footer<G>)]
pub fn footer() -> Template<G> {
    template! {
        footer(class="mt-2 bg-gray-100 text-center") {
            p(class="font-bold") { "Hacker News Clone" }
            p {
                "Powered by "
                a(class="underline", href="https://sycamore-rs.netlify.app", target="_blank") { "Sycamore" }
            }
        }
    }
}
