mod lichess;
mod minimax;
mod constants;

use lichess::make_moves_in_all_ongoing_games;
use lichess::respond_to_all_challenges;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

/// This is the format the labmda function expects the request to be in.
/// If you want to accept commands in the request, you will need to update this
#[derive(Deserialize)]
struct Request {
    
}

/// This defines what the response structure should look like
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the function that will be called when the lambda is invoked
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    dotenv().ok();
    respond_to_all_challenges().await?;
    make_moves_in_all_ongoing_games().await?;    

    Ok(Response {
        req_id: event.context.request_id,
        msg: format!("Success"),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}