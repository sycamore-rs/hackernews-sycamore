use sycamore::prelude::*;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    sycamore::render(|| {
        template! {
            div(class="bg-red-300") {
                "Sycamore Hacker News"
            }
            "Hello World!"
        }
    });
}
