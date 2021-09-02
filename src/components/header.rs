use sycamore::prelude::*;

#[component(NavLink<G>)]
pub fn nav_link((href, title): (&'static str, &'static str)) -> Template<G> {
    static LINK_CLASS: &str =
        "flex-1 text-center rounded hover:rounded-md hover:bg-gray-200 px-2 py-1 transition";

    template! {
        a(class=LINK_CLASS, href=href) { (title) }
    }
}

#[component(Header<G>)]
pub fn header() -> Template<G> {
    template! {
        nav(class="flex flex-row space-x-2 py-2") {
            h1(class="font-bold py-1") { "Hacker News Clone" }

            NavLink(("/", "Top"))
            NavLink(("/new", "Latest"))
            NavLink(("/best", "Best"))
            NavLink(("/show", "Show"))
        }
    }
}
