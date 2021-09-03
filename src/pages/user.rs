use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::UserData;
use crate::components::show_stories::ShowStories;

#[component(User<G>)]
pub fn user(props: Result<UserData>) -> Template<G> {
    match props {
        Ok(user) => {
            let UserData {
                id,
                karma,
                about,
                submitted: _,
                stories,
            } = user;
            template! {
                h1(class="text-xl font-semibold") { (id) }
                p(class="text-sm text-gray-600") { (karma) " karma" }
                p(class="mb-2") { (about) }
                ShowStories(stories)
            }
        }
        Err(_) => template! {
            "Error fetching user."
        },
    }
}
