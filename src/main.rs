use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};
use std::env;
use pretty_env_logger;
use http:StatusCode;

pub struct server
{
    pub status: String,
    pub health: String,
    pub ping: String,
    pub polynomial: String,
    pub differentiate: String,
    pub integrate: String,
    pub vector: String,
}

pub async fn reponse_router() -> WebResult<impl Reply>
{
    let response_json = &solution_options
    {
        status: health().to_string(),
        health: status().to_string(),
        ping: ping().to_string(),
    };
    Ok(json(reponse_json));
}

async fn main()
{
if std::env::var_os("RUST_LOG").is_none()
{
    std::env::set_var("RUST_LOG", "api=info");
}
pretty_env_logger::init();

let health = warp::path!("api" / "health").and(warp::get()).and_then(health);

let status = warp::path!("api" / "status").and(warp::get()).and_then(status);

let ping = warp::path!("api" / "ping").and(warp::get()).and_then(ping);

let routes = server.with(warp::log("api"));

println!("Server online. Confirm status.");
warp::serve(routes).run((0, 0, 0, 0), 8000).await;
}

fn health() -> String
{
    pub struct request_status
    {
        assert_eq!(StatusCode::OK.as_u16(), 200);
        assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
        assert!(StatusCode::OK.is_success());
    }
    let status = &request_status.stringify();
    if status == "StatusCode::OK"
    {
        return "Server Healthy";
    }
    else
    {
        return "Server Error";
    }
}

fn status()
{
// Add later using HTTP error codes and router to return server status using API model and
// handler.rs file 
}

fn ping()
{
// Add later when you actually figure out what to do.
}
