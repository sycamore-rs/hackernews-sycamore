use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::StoryPageData;
use crate::components::comment::CommentView;
use crate::components::show_stories::Story;

#[component(Item<G>)]
pub fn item(props: Result<StoryPageData>) -> View<G> {
    match props {
        Ok(story) => {
            let StoryPageData { item, comments } = story;

            let has_comments = !comments.is_empty();
            let comments_view = View::new_fragment(
                comments
                    .into_iter()
                    .map(|comment| {
                        view! {
                            CommentView(comment)
                        }
                    })
                    .collect(),
            );
            view! {
                ul(class="list-none mb-2") {
                    Story(item)
                }
                ul(class="list-none") {
                    (if has_comments {
                        comments_view.clone()
                    } else {
                        view! { "No comments yet." }
                    })
                }
            }
        }
        Err(_) => view! {
            "Error fetching item."
        },
    }
}
