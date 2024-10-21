use axum::extract::Path;
use axum::handler::Handler;
use axum::http::{Response, StatusCode};
use axum::response::Html;
use axum::Router;
use axum::routing::{get, post};
// use utils::controller;



pub fn routes() -> Router {
    Router::new()
        // .merge(page_routes())
        .merge(service_routes())
    // service_routes(router)
}

// fn page_routes() -> Router {
//     Router::new()
//         // .route("/", get(page::menu))
//         .route("/menu", get(page::menu))
// }


fn service_routes() -> Router {
    Router::new()
        // .route("/", get(service::menu))
        // .route("/menu", get(service::menu))
        // .route("/system", get(controller::collection::system))
        // .route("/voice/list", post(controller::collection::get_voice_list))
        // .route("/voice/upload", post(controller::collection::upload_voice))
        // .route("/voice/delete", post(controller::collection::delete_voice))
        // .route("/voice/download", post(controller::collection::get_voice))
        // .route("/voice/info", post(controller::collection::get_voice_info))
}
