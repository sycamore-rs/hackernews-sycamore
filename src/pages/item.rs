use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::StoryPageData;
use crate::components::comment::CommentView;
use crate::components::show_stories::Story;

#[component(Item<G>)]
pub fn item(props: Result<StoryPageData>) -> Template<G> {
    match props {
        Ok(story) => {
            let StoryPageData { item, comments } = story;

            let has_comments = !comments.is_empty();
            let comments_view = Template::new_fragment(
                comments
                    .into_iter()
                    .map(|comment| {
                        template! {
                            CommentView(comment)
                        }
                    })
                    .collect(),
            );
            template! {
                ul(class="list-none mb-2") {
                    Story(item)
                }
                ul(class="list-none") {
                    (if has_comments {
                        comments_view.clone()
                    } else {
                        template! { "No comments yet." }
                    })
                }
            }
        }
        Err(_) => template! {
            "Error fetching item."
        },
    }
}
