use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::UserData;
use crate::components::show_stories::ShowStories;

#[component(User<G>)]
pub fn user(props: Result<UserData>) -> View<G> {
    match props {
        Ok(user) => {
            let UserData {
                id,
                karma,
                about,
                submitted: _,
                stories,
            } = user;
            view! {
                h1(class="text-xl font-semibold") { "User: " (id) }
                p(class="text-sm text-gray-600") { (karma) " karma" }
                p(class="mb-2") { (about) }
                ShowStories(stories)
            }
        }
        Err(_) => view! {
            "Error fetching user."
        },
    }
}
