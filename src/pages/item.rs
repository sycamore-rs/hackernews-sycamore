use sycamore::prelude::*;

use crate::apis::types::StoryPageData;
use crate::components::comment::CommentView;
use crate::components::show_stories::Story;

#[component(inline_props)]
pub async fn Item<G: Html>(cx: Scope<'_>, id: i64) -> View<G> {
    let data = crate::apis::get_story(id).await;
    match data {
        Ok(story) => {
            let StoryPageData { item, comments } = story;

            let has_comments = !comments.is_empty();
            let comments_view = View::new_fragment(
                comments
                    .into_iter()
                    .map(|comment| {
                        view! { cx,
                            CommentView(comment=comment)
                        }
                    })
                    .collect(),
            );
            view! { cx,
                ul(class="list-none mb-2") {
                    Story(story=item)
                }
                ul(class="list-none") {
                    (if has_comments {
                        comments_view.clone()
                    } else {
                        view! { cx, "No comments yet." }
                    })
                }
            }
        }
        Err(_) => view! { cx,
            "Error fetching item."
        },
    }
}
