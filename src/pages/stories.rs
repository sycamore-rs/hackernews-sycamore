use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::StoryItem;
use crate::components::show_stories::ShowStories;

#[component(Stories<G>)]
pub fn stories(props: Result<Vec<StoryItem>>) -> Template<G> {
    match props {
        Ok(stories) => template! {
            ShowStories(stories)
        },
        Err(_) => template! {
            "Error fetching stories."
        },
    }
}
