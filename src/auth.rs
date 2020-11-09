use serde_derive::{Deserialize, Serialize};
use std::env;
use warp::{
    http::StatusCode,   Filter,
};

#[derive(Deserialize, Serialize)]
struct Auth{
    code: String,
    state: String
}

pub async fn listen_for_code() {
    let route = warp::get()
        .and(warp::path("auth"))
        .and(warp::query::<Auth>())
        .map(|r: Auth|{
            env::set_var("MAL_CODE", r.code);
            env::set_var("MAL_STATE", r.state);
            StatusCode::OK
        });
    
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}