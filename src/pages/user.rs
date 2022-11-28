use sycamore::prelude::*;

use crate::apis::types::UserData;
use crate::components::show_stories::ShowStories;

#[component(inline_props)]
pub async fn User<G: Html>(cx: Scope<'_>, username: String) -> View<G> {
    let data = crate::apis::get_user_page(&username).await;
    match data {
        Ok(user) => {
            let UserData {
                id,
                karma,
                about,
                submitted: _,
                stories,
            } = user;
            view! { cx,
                h1(class="text-xl font-semibold") { "User: " (id) }
                p(class="text-sm text-gray-600") { (karma) " karma" }
                p(class="mb-2") { (about) }
                ShowStories(stories=stories)
            }
        }
        Err(_) => view! { cx,
            "Error fetching user."
        },
    }
}
