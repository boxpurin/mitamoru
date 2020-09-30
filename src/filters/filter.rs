use warp::Filter;

pub fn hello() -> warp::filters::BoxedFilter<()>{
  warp::path("hello").boxed()
}

pub fn name() -> warp::filters::BoxedFilter<(String,)>{
  warp::path::param().boxed()
}