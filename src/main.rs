use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};
use tokio

pub struct solution_options
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
        status: health_status_ping_checker(status).to_string(),
        health: health_status_ping_checker(health).to_string(),
        ping: health_status_ping_checker(ping).to_string(),
        polynomial: main(equation).to_string(),
        differentiate: main(equation).to_string(),
        integrate: main(equation).to_string(),
        vector: main(equation).to_string(),
    };
    Ok(json(reponse_json));
}

async fn main()
{

}

pub fn health_status_ping_checker(query: String)
{
    
}

pub fn polynomialengine()
{

}

pub fn differentiationengine()
{

}

pub fn integrationengine()
{

}

pub fn vectorengine()
{

}
