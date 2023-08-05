
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

mod int_program;
mod adventure;

#[derive(Deserialize)]
struct Request {
    command: Vec<String>,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: Vec<String>,
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let commands = event.payload.command;
    let text = adventure::advanture(commands);
    
    let resp = Response {
        req_id: event.context.request_id,
        msg: text,
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .without_time()
        .init();
    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

// fn main() {
//     let text = adventure::advanture(vec![]);
//     println!("{:?}", text)
// }