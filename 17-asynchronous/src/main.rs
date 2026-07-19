use std::{result, thread};
use std::time::Duration;
use trpl::{Either, Html}; // trpl is short fot "The Rust Programming Language", it's just a crate containing
// utilities for this project for managing async stuff (it uses `futures` and `tokio`,
// two famous crates)

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await; // Futures are *lazy* in Rust, meaning they don't
    // do anything unless you consume with `await`
    let response_text = response.text().await; // we wait for the response to come, but also
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
    (
        url,
        title.map(|title| title.replace(" ", "").replace("\n", "")),
    )
}

fn main() {
    // ____________________________________________________________________________________________
    // To run Async functions, we need an Async Runtime, provided by a library
    // The one used here is the `tokio` runtime, wrapped in the  `trpl` crate under a function named `block_on`
    // ____________________________________________________________________________________________

    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_future1 = page_title_with_url(&args[1]);
        let title_future2 = page_title_with_url(&args[2]);

        let (url, maybe_title) = match trpl::select(title_future1, title_future2).await {
            // this returns the result of the first one that completes
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    });

    // ____________________________________________________________________________________________
    // Let's implement a behavior similar to the one of chapter 16, with two parallel tasks
    // ____________________________________________________________________________________________

    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("Task 1: {i}");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("Task 2: {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });


    // ____________________________________________________________________________________________
    // Now let's implement some message passing using channels (like with threads)
    // ____________________________________________________________________________________________

    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel(); // differently than with threads, the receiver is mutable

        let f1 = async move {
            // N.B: why we are using `move` here?
            let vals = vec![
                // The reason is that the program wouldn't exit otherwise.
                String::from("hello"), // This is because the `while` loop of the second block
                String::from("from"),  // only exits when a None is received.
                String::from("Rust"),  // But a None is only received when `tx.close()` is called.
                String::from("language!"), // Since the ownership of tx would be of the wrapping block,
            ]; // we need to move it inside this async block so that
            // the `close` method is called once this block is completed.
            for val in vals {
                tx.send(val).unwrap(); // the `send` is non-blocking
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let f2 = async {
            // new construct here! This loop continues until the right part matches the pattern
            while let Some(v) = rx.recv().await {
                println!("Received '{v}'!");
            }
        };

        trpl::join(f1, f2).await;
        println!("--------------------------------------------------");

        // If we need to have multiple producers, we just need to clone the tx
        let (tx, mut rx) = trpl::channel(); // differently than with threads, the receiver is mutable

        let tx1 = tx.clone();

        let f1 = async move {
            let vals = vec![
                String::from("hello"),
                String::from("from"),
                String::from("Rust"),
                String::from("language!"),
            ];

            for val in vals {
                tx.send(val).unwrap(); // the `send` is non-blocking
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let f2 = async {
            // new construct here! This loop continues until the right part matches the pattern
            while let Some(v) = rx.recv().await {
                println!("Received '{v}'!");
            }
        };

        let f3 = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("yeah"),
                String::from("!"),
            ];

            for val in vals {
                tx1.send(val).unwrap(); // the `send` is non-blocking
                trpl::sleep(Duration::from_millis(750)).await;
            }
        };

        trpl::join!(f1, f2, f3);  // join! macro is used when an arbitrary number of arguments is specified
    });

    // ____________________________________________________________________________________________
    // Let's try implement a `timeout` function that gives an async block a maximum time to complete!
    // ____________________________________________________________________________________________

    trpl::block_on(async {

        async fn timeout<F: Future>(f: F, max_time: Duration) -> Result<F::Output, Duration> {
            match trpl::select(f, trpl::sleep(max_time)).await {
                Either::Left(output) => Ok(output),
                Either::Right(_) => Err(max_time)
            }
        }

        let result = timeout(async {
            trpl::sleep(Duration::from_millis(500)).await
        }, Duration::from_secs(1)).await;

        println!("This should be ok: {result:?}");

        let result = timeout(async {
            trpl::sleep(Duration::from_millis(1200)).await
        }, Duration::from_secs(1)).await;

        println!("This should be error: {result:?}")
    });

    // ____________________________________________________________________________________________
    // Streams
    // ____________________________________________________________________________________________

    // Streams are a concept that puts Iterator and Future together. While calling Iterator::next()
    // function is synchronous, calling Receiver::recv() is asynchronous. But we can think of iterators
    // like a future obtaining a list of values in sequence, like a data transfer over the network

    // Streams are not yet part of the Rust standard library, but the trpl provides one:

    use trpl::StreamExt;  // StreamExt is an extending trait, a common pattern of Rust where a Trait
                          // extends another, in this case StreamExt provides a higher level API than Stream.

    trpl::block_on(async {
        let vals = vec![1,2,3,4,5,6,7];
        let iter = vals.iter().map(|x| x*2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("Received value {value}");
        }
    });

}
