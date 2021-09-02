use sycamore::prelude::*;

#[component(Header<G>)]
pub fn header() -> Template<G> {
    template! {
        nav(class="flex flex-row space-x-2 justify-evenly py-2") {
            h1(class="font-bold") { "Hacker News Clone" }

            a(href="/") { "Top" }
            a(href="/new") { "Latest" }
            a(href="/best") { "Best" }
            a(href="/show") { "Show" }
        }
    }
}
