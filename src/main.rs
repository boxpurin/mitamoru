use warp::Filter;
use warp::{Reply, Rejection};

#[tokio::main]
async fn main() {
    let hi = warp::path("hi").map(|| "hi.");
    
    // Match any request and return hello world!
    let routes = warp::get()
    .and(
        hi.or(
            hello().and(name()).and_then(greet_handler)
        )
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn hello() -> warp::filters::BoxedFilter<()>{
    warp::path("hello").boxed()
}

fn name() -> warp::filters::BoxedFilter<(String,)>{
    warp::path::param().boxed()
}

async fn greet_handler(name:String) -> Result<impl Reply, Rejection>{
    let reply = format!("hello {}", name);
    Ok(warp::reply::html(reply))
}

