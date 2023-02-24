use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    loops: i32,
    count: i32,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn sum(count: i32) -> Result<(), Error> {
    let mut numbers = Vec::new();
    for i in 0..count {
        numbers.push(i);
    }
    let sum = numbers.par_iter().sum::<i32>();
    //print the sum and the id of each thread
    println!("Sum: {} Thread ID: {:?}", sum, std::thread::current().id());
    Ok(())
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let count = event.payload.count;
    let loops = event.payload.loops;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: "completed.".to_string(),
    };

    // run the sum function
    for i in 0..loops {
        let benchmark = sum(count).await;
        println!("Benchmark: {:?}", benchmark);
    }
    

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
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