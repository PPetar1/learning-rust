// We make a dependency to trpl (The Rust Programming Language) which re-exports some types
// traits and functions from various crates, primarly futures and tokio. future crate is for async
// code and tokio is most widely used async runtime in Rust


// Main difference between async and standard thread concurency is that threads use 
// operating system threads while async yields when awaiting and gives other parts of code chance
// to continue executing while we wait (this is implemented with runtimes like tokio). Async 
// is helpful for I/O bound operations while for CPU bound threads or methods like
// tokio::task::spawn_blocking should be used
// https://rust-for-c-programmers.com/ch22/22_3_choosing_the_right_model_threads_vs_async_for_i_o_bound_vs_cpu_bound_tasks.html

use trpl::{Html, Either};
use std::future::Future;

//async fn main() { // This wont compile, main cannot be async! Runtime needs to be initialized
                    // before async is used
fn main() {
    let args: Vec<String> = std::env::args().collect();
   
    trpl::block_on(async { // This calls tokio block_on method which takes a future and blocks
                           // until it is resolved. Async block wraps the return value into a
                           // future i think
         let title_fut_1 = page_title(&args[1]);
         let title_fut_2 = page_title(&args[2]);// We create two futures here and race them to see
                                                // which one finishes first

         let (url, maybe_title) = 
             match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
             };

         println!("{url} returned first");
         

         match maybe_title {
             Some(title) => println!("The title was {title}"),
             None => println!("There was no title"),
         }
    });
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;// We use await on the future to get its response. Futures
                                        // in Rust are lazy and until you use await, they will not
                                        // execute their code.
    let response_text = response.text().await;// I think we await here as well because response can
                                              // be partially passed or something like that
    let title = Html::parse(&response_text)// We parse the response and get back Html type
        .select_first("title")// We select first occurence of <title> element
        .map(|title| title.inner_html());// We use Option::map here and map the result if present to
                                        // title.inner_html() which gives us the String content so
                                        // at the end we have a Option<String here>

    (url, title)
}



fn page_title_(url: &str) -> impl Future<Output = Option<String>> {// This is roughly equivalent to
                                                                   // what the async function above
                                                                   // will be compiled to
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

//Each await point—that is, every place where the code uses the await keyword—represents a place where control is handed back to the runtime. To make that work, Rust needs to keep track of the state involved in the async block so that the runtime could kick off some other work and then come back when it’s ready to try advancing the first one again.
