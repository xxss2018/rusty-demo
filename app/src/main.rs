// use std::os::unix::net::SocketAddr;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Router,
};

use axum_extra::TypedHeader;
use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::extract::{connect_info::ConnectInfo, ws::CloseFrame};

use futures::{sink::SinkExt, stream::StreamExt};
use axum::http::StatusCode;
use axum::response::Html;
use axum::extract::Path;
use std::fs::File;
use std::io::Read;
use axum::extract::{State, Extension};
use std::sync::Arc;
use tokio::sync::Mutex;
// mod route;
// mod service;
// use service::hello::handler;
mod router;
use tklog::{LOG,LEVEL};


use std::result::Result;
use sqlx::{
    Postgres,
    postgres::PgPoolOptions, migrate::MigrateDatabase,
};
use axum::Error;
use core::database::pgpool::Db;



#[tokio::main]
async fn main() -> Result<(), Error>{
    LOG.set_level(LEVEL::Debug);
    let pool = Db::new().await.unwrap();
    // let ext = Arc::new(pool);
    let app: Router = Router::new()
        .merge(router::routes())
        .layer(Extension(pool));
        
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9000")
        .await.unwrap();
    axum::serve(listener,app).await.unwrap();
    Ok(())

}
