mod models;
mod data;
mod utils;
mod utils_tests;
use warp::Filter;

#[tokio::main]
async fn main() {
    #[cfg(test)]

    let route = warp::post()
        .and(warp::path("check_availability"))
        .and(warp::body::json())
        .and_then(utils::handle_availability_request);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
