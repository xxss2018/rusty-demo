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



use std::result::Result;
use sqlx::{
    Postgres,
    postgres::PgPoolOptions, migrate::MigrateDatabase,
};
use axum::Error;

pub struct Db {
    pub pool: sqlx::Pool<Postgres>
}

const DB_URL:&str = "postgres://postgres:mysecretpassword@localhost/postgres";

impl Db {
    pub async fn new() -> Result<sqlx::Pool<Postgres>, Error> {
        if !Postgres::database_exists(DB_URL).await.unwrap_or(false) {
            // Postgres::create_database(DB_URL).await.unwrap();
            match Postgres::create_database(DB_URL).await {
                Ok(_) => {
                    println!("create database success");
                },
                Err(e) => {
                    println!("create database error: {:?}", e);
                    return Err(Error::new(""));
                }
            }
        } else {
            println!("database already exists");
        }
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(DB_URL).await {
            Ok(pool) => {
                println!("connect database success");
                return Ok(pool);
            },
            Err(e) => {
                println!("connect database error: {:?}", e);
                return Err(Error::new(""));
            }
        }
    }
}

