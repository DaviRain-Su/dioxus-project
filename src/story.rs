use chrono::{DateTime, Utc};
use dioxus::prelude::Readable;
use dioxus::prelude::*;
use dioxus::{core_macro::component, dioxus_core::Element, signals::ReadOnlySignal};
use serde::{Deserialize, Serialize};

use crate::fetching_data::get_story;
use crate::PreviewState;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

impl Default for StoryItem {
    fn default() -> Self {
        Self {
            id: 0,
            title: "Hello hackernews".to_string(),
            url: None,
            text: None,
            by: "Author".to_string(),
            score: 0,
            descendants: 0,
            time: chrono::Utc::now(),
            kids: vec![],
            r#type: "".to_string(),
        }
    }
}

async fn resolve_story(
    mut full_story: Signal<Option<StoryPageData>>,
    mut preview_state: Signal<PreviewState>,
    story_id: i64,
) {
    if let Some(cached) = full_story.as_ref() {
        *preview_state.write() = PreviewState::Loaded(cached.clone());
        return;
    }

    *preview_state.write() = PreviewState::Loading;
    if let Ok(story) = get_story(story_id).await {
        *preview_state.write() = PreviewState::Loaded(story.clone());
        *full_story.write() = Some(story)
    }
}

#[component]
pub fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let mut preview_state = consume_context::<Signal<PreviewState>>();
    let StoryItem {
        title,
        by,
        url,
        score,
        time,
        kids,
        id,
        ..
    } = story();

    let full_story = use_signal(|| None);

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    let score = format!("{score} {}", if score == 1 { "poins" } else { "points" });
    let comments = format!(
        "{}{}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            onmouseenter: move |_event| {
                resolve_story(full_story, preview_state, id)
            },
            div { font_size: "1.5rem",
                a { href: url,
                    onfocus: move |_event| {
                        resolve_story(full_story, preview_state, id)
                    },
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div { display: "flex", flex_direction: "row", color: "gray",
                div { "{score}"}
                div { padding_left: "0.5rem", "by {by}"}
                div { padding_left: "0.5rem", "{time}"}
                div { padding_left: "0.5rem", "{comments}"}
            }
        }
    }
}
