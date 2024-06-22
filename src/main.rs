#![allow(non_snake_case)]

use dioxus::prelude::*;
use fetching_data::get_stories;
use story::Comment;
use story::StoryPageData;
use tracing::Level;

pub mod error;
pub mod fetching_data;
pub mod story;

use story::StoryListing;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));

    rsx! {
        div {
            display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}

fn Stories() -> Element {
    let stories = use_resource(move || get_stories(10));

    match &*stories.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    for story in list {
                        StoryListing { story: story.clone() }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { "An error occurred while fetching stories {err}"}
        }
        None => {
            rsx! { "Loading items" }
        }
    }
}

#[derive(Debug, Clone)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

fn Preview() -> Element {
    let preview_state = consume_context::<Signal<PreviewState>>();
    match preview_state() {
        PreviewState::Unset => rsx! {
            "Hover over a story to preview it here"
        },
        PreviewState::Loading => rsx! { "Loading..." },
        PreviewState::Loaded(story) => {
            rsx! {
                div { padding: "0.5rem",
                    div { font_size: "1.5rem", a { href: story.item.url, "{story.item.title}"}}
                    div { dangerous_inner_html: story.item.text }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn Comment(comment: Comment) -> Element {
    rsx! {
        div { padding: "0.5rem",
            div { color: "gray", "by {comment.by}"}
            div { dangerous_inner_html: "{comment.text}" }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}
