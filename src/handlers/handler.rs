use warp::{Reply, Rejection};

pub async fn greet_handler(name:String) -> Result<impl Reply, Rejection>{
  let reply = format!("hello {}", name);
  Ok(warp::reply::html(reply))
}