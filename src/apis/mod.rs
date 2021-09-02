pub mod types;

use anyhow::Result;
use futures::future::join_all;
use reqwasm::http::Request;
use types::*;

/// Hacker News API url.
pub static BASE_API_URL: &str = "https://hacker-news.firebaseio.com/v0/";
pub static TOP_STORIES: &str = "topstories.json";
pub static NEW_STORIES: &str = "newstories.json";
pub static BEST_STORIES: &str = "beststories.json";
pub static SHOW_STORIES: &str = "showstories.json";
pub static ITEM_API: &str = "item/";
pub static USER_API: &str = "user";

const STORIES_COUNT: usize = 30;
const COMMENT_DEPTH: i64 = 3;

pub async fn get_story(id: i64) -> Result<StoryItem> {
    let url = format!("{}{}{}.json", BASE_API_URL, ITEM_API, id);
    Ok(Request::get(&url).send().await?.json().await?)
}

pub async fn get_stories(story_sorting: StorySorting) -> Result<Vec<StoryItem>> {
    let stories_api = match story_sorting {
        StorySorting::Top => TOP_STORIES,
        StorySorting::New => NEW_STORIES,
        StorySorting::Best => BEST_STORIES,
        StorySorting::Show => SHOW_STORIES,
    };
    let url = format!("{}{}", BASE_API_URL, stories_api);
    let stories_ids = Request::get(&url).send().await?.json::<Vec<i64>>().await?;

    let story_futures = stories_ids[..usize::min(stories_ids.len(), STORIES_COUNT)]
        .iter()
        .map(|&story_id| get_story(story_id));
    let stories = join_all(story_futures)
        .await
        .into_iter()
        .filter_map(|story| story.ok())
        .collect();
    Ok(stories)
}
