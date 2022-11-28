mod apis;
mod components;
mod pages;

use apis::types::StorySorting;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

#[derive(Debug, Route)]
enum AppRoutes {
    #[to("/")]
    Top,
    #[to("/new")]
    New,
    #[to("/best")]
    Best,
    #[to("/show")]
    Show,
    #[to("/user/<username>")]
    User { username: String },
    #[to("/item/<id>")]
    Item { id: i64 },
    #[not_found]
    NotFound,
}

#[component(inline_props)]
async fn Switch<'a, G: Html>(cx: Scope<'a>, route: &'a ReadSignal<AppRoutes>) -> View<G> {
    view! { cx,
        (match route.get().as_ref() {
            AppRoutes::Top => view! { cx, pages::stories::Stories(sorting=StorySorting::Top) },
            AppRoutes::New => view! { cx, pages::stories::Stories(sorting=StorySorting::New) },
            AppRoutes::Best => view! { cx, pages::stories::Stories(sorting=StorySorting::Best) },
            AppRoutes::Show => view! { cx, pages::stories::Stories(sorting=StorySorting::Show) },
            AppRoutes::User { username } => view! { cx, pages::user::User(username=username.clone()) },
            AppRoutes::Item { id } => view! { cx, pages::item::Item(id=*id) },
            AppRoutes::NotFound => view! { cx, "404 Page Not Found"}
        })
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx: Scope, route: &ReadSignal<AppRoutes>| view! { cx,
                div(class="app mb-2") {
                    components::header::Header()
                    div(class="container mx-auto") {
                        Switch(route=route)
                    }
                }
            }
        )
    }
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    sycamore::render(App);
}
