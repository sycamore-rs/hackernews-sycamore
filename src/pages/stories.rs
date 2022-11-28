use sycamore::prelude::*;

use crate::apis::types::StorySorting;
use crate::components::show_stories::ShowStories;

#[component(inline_props)]
pub async fn Stories<G: Html>(cx: Scope<'_>, sorting: StorySorting) -> View<G> {
    let data = crate::apis::get_stories(sorting).await;
    match data {
        Ok(stories) => view! { cx,
            ShowStories(stories=stories)
        },
        Err(_) => view! { cx,
            "Error fetching stories."
        },
    }
}
