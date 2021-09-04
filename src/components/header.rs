use sycamore::prelude::*;

#[component(NavLink<G>)]
pub fn nav_link((href, title): (&'static str, &'static str)) -> Template<G> {
    template! {
        a(
            class="text-center font-bold text-gray-200 hover:text-gray-100 transition",
            href=href,
        ) {
            (title)
        }
    }
}

#[component(Header<G>)]
pub fn header() -> Template<G> {
    template! {
        div(class="bg-green-400 mb-3") {
            nav(class="container flex flex-row space-x-4 mx-auto py-4") {
                NavLink(("", "HN"))
                NavLink(("new", "Latest"))
                NavLink(("best", "Best"))
                NavLink(("show", "Show"))
                div(class="flex-1") // spacer
                a(href="https://sycamore-rs.netlify.app", target="_blank", class="font-light text-gray-200") {
                    "Built with Sycamore"
                }
            }
        }
    }
}
