use trpl::{Either, Html}; // trpl is short fot "The Rust Programming Language", it's just a crate containing
                // utilities for this project for managing async stuff (it uses `futures` and `tokio`,
                // two famous crates)

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;  // Futures are *lazy* in Rust, meaning they don't
                                                    // do anything unless you consume with `await`
    let response_text = response.text().await;  // we wait for the response to come, but also
                                                       // for the entirety of the body to arrive
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

// When Rust sees an `async` block, it compiles it to a non-async block, roughly like this:

fn page_title_2(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

async fn page_title_with_url(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title.map(|title| title.replace(" ", "").replace("\n", "")))
}

fn main() {
    // To run Async functions, we need an Async Runtime, provided by a library
    // The one used here is the `tokio` runtime, wrapped in the  `trpl` crate under a function named `block_on`

    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_future1 = page_title_with_url(&args[1]);
        let title_future2 = page_title_with_url(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_future1, title_future2).await { // this returns the result of the first one that completes
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }

    })
}
