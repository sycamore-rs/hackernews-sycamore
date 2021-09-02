use sycamore::prelude::*;

#[component(Header<G>)]
pub fn header() -> Template<G> {
    static LINK_CLASS: &str =
        "flex-1 text-center rounded hover:rounded-md hover:bg-gray-200 px-2 py-1 transition";

    template! {
        nav(class="flex flex-row space-x-2 py-2") {
            h1(class="font-bold py-1") { "Hacker News Clone" }

            a(class=LINK_CLASS, href="/") { "Top" }
            a(class=LINK_CLASS, href="/new") { "Latest" }
            a(class=LINK_CLASS, href="/best") { "Best" }
            a(class=LINK_CLASS, href="/show") { "Show" }
        }
    }
}
