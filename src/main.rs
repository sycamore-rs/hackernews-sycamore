mod apis;
mod components;
mod pages;

use apis::types::StorySorting;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};
use wasm_bindgen_futures::spawn_local;

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
    User(String),
    #[to("/item/<id>")]
    Item(i64),
    #[not_found]
    NotFound,
}

#[component(App<G>)]
fn app() -> Template<G> {
    template! {
        Router(RouterProps::new(HistoryIntegration::new(), |route: StateHandle<AppRoutes>| {
            let template = Signal::new(Template::empty());
            create_effect(cloned!((template) => move || {
                let route = route.get();
                spawn_local(cloned!((template) => async move {
                    let t = match route.as_ref() {
                        AppRoutes::Top  => {
                            let data = apis::get_stories(StorySorting::Top).await;
                            template! {
                                pages::stories::Stories(data)
                            }
                        },
                        AppRoutes::New  => {
                            let data = apis::get_stories(StorySorting::New).await;
                            template! {
                                pages::stories::Stories(data)
                            }
                        },
                        AppRoutes::Best  => {
                            let data = apis::get_stories(StorySorting::Best).await;
                            template! {
                                pages::stories::Stories(data)
                            }
                        },
                        AppRoutes::Show => {
                            let data = apis::get_stories(StorySorting::Show).await;
                            template! {
                                pages::stories::Stories(data)
                            }
                        },
                        AppRoutes::User(username) => {
                            let data = apis::get_user_page(&username).await;
                            template! {
                                pages::user::User(data)
                            }
                        },
                        AppRoutes::Item(id) => {
                            let data = apis::get_story(*id).await ;
                            template! {
                                pages::item::Item(data)
                            }
                        },
                        AppRoutes::NotFound => template! {
                            "Page not found."
                        },
                    };
                    template.set(t);
                }));
            }));

            template! {
                div(class="app mb-2") {
                    components::header::Header()
                    div(class="container mx-auto") {
                        (template.get().as_ref())
                    }
                }
            }
        }))
    }
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    sycamore::render(|| {
        template! {
            App()
        }
    });
}
