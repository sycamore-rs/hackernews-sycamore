use sycamore::prelude::*;

use crate::apis::types::Comment;

#[component(CommentView<G>)]
pub fn comment_view(comment: Comment) -> Template<G> {
    let Comment {
        id: _,
        by,
        text,
        time,
        kids: _,
        sub_comments,
        r#type: _,
    } = comment;

    let by_url = format!("/user/{}", by);

    let sub_comments_view = Template::new_fragment(
        sub_comments
            .into_iter()
            .map(|comment| {
                template! {
                    CommentView(comment)
                }
            })
            .collect(),
    );

    template! {
        li(class="mt-2") {
            div(class="mb-2 text-gray-600 border-t border-gray-300") {
                a(href=by_url) { (by) } " | " span { (time.format("%D %l:%M %p")) }
            }
            p(dangerously_set_inner_html=&text)
            ul(class="list-none ml-5") {
                (sub_comments_view)
            }
        }
    }
}
