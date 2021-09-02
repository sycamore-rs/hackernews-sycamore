use anyhow::Result;
use sycamore::prelude::*;

use crate::apis::types::StoryItem;

#[component(Story<G>)]
pub fn story(story: StoryItem) -> Template<G> {
    let StoryItem {
        id,
        title,
        url,
        text,
        by,
        score,
        descendants,
        time,
        kids,
        r#type,
    } = story;

    // TODO: user view in app
    let by_url = format!("https://news.ycombinator.com/user?id={}", by);
    let kids_url = format!("https://news.ycombinator.com/item?id={}", id);
    let kids_len = kids.len();

    template! {
        li(class="rounded border border-gray-300 p-2") {
            div {
                a(href=url.as_deref().unwrap_or_default(), target="_blank", rel="noreferrer") { (title) }
            }
            div(class="text-sm text-gray-600") {
                span {
                    (score) (if score == 1 { " point" } else { " points" })
                }
                span {
                    " by " a(href=by_url, target="_blank", rel="noreferrer") { (by) }
                }
                " | " span { (time.format("%D %l:%M %p")) } " | "
                span {
                    a(href=kids_url, target="_blank", rel="noreferrer") {
                        (kids_len) (if kids_len == 1 { " comment"} else { " comments" })
                    }
                }
            }
        }
    }
}

#[component(ShowStories<G>)]
pub fn show_stories(stories: Result<Vec<StoryItem>>) -> Template<G> {
    match stories {
        Ok(stories) => {
            let list = Template::new_fragment(
                stories
                    .into_iter()
                    .map(|item| {
                        template! {
                            Story(item)
                        }
                    })
                    .collect(),
            );
            template! {
                ul(class="list-none space-y-2") {
                    (list)
                }
            }
        }
        _ => {
            template! {
                "Error fetching stories."
            }
        }
    }
}
