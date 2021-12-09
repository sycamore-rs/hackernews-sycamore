use sycamore::prelude::*;
use web_sys::Url;

use crate::apis::types::StoryItem;

#[component(Story<G>)]
pub fn story(story: StoryItem) -> View<G> {
    let StoryItem {
        id,
        title,
        url,
        text: _,
        by,
        score,
        descendants: _,
        time,
        kids,
        r#type: _,
    } = story;
    let hostname = Url::new(url.as_deref().unwrap_or_default())
        .map(|url| {
            let mut hostname = url.hostname();
            if hostname.starts_with("www.") {
                hostname = hostname[4..].to_string();
            }
            hostname
        })
        .ok();

    // TODO: user view in app
    let by_url = format!("user/{}", by);
    let kids_url = format!("item/{}", id);
    let kids_len = kids.len();

    view! {
        li(class="rounded border border-gray-300 p-1") {
            div {
                a(href=url.as_deref().unwrap_or_default(), target="_blank", rel="noreferrer", class="font-semibold") {
                    (title)
                }
                (if let Some(hostname) = hostname.clone() {
                    view! { span(class="text-gray-600 text-sm") { " (" (hostname) ")" } }
                } else {
                    view! {}
                })
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
pub fn show_stories(stories: Vec<StoryItem>) -> View<G> {
    let list = View::new_fragment(
        stories
            .into_iter()
            .map(|item| {
                view! {
                    Story(item)
                }
            })
            .collect(),
    );
    view! {
        ul(class="list-none space-y-2") {
            (list)
        }
    }
}
