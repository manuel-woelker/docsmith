use crate::server_error::ServerError;
use axum::Router;
use axum::routing::get;
use docsmith_base::result::DocsmithResult;
use http::{Response, StatusCode};
use std::borrow::Cow;
use std::convert::Infallible;
use std::time::Duration;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use futures_util::Stream;
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self};
use tokio::sync::broadcast;
use tokio::{task, time};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};

pub struct DocsmithServer {}

const LIVE_SERVICE_HTML: &str = include_str!("assets/live_service.html");
const LIVE_SERVICE_JS: &str = include_str!("assets/live_service.js");

impl DocsmithServer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self) -> DocsmithResult<()> {
        let (tx, mut rx1) = broadcast::channel::<String>(16);
        let tx2 = tx.clone();
        // build our application with a single route
        //        let app = Router::new().route("/", get(index_handler));
        let app = Router::new()
            .route(
                "/",
                get(async || -> Result<_, ServerError> {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Cow::Borrowed(LIVE_SERVICE_HTML).to_string())?)
                }),
            )
            .route(
                "/live_service.js",
                get(async || -> Result<_, ServerError> {
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Cow::Borrowed(LIVE_SERVICE_JS).to_string())?)
                }),
            )
            .route(
                "/events2",
                get(sse_handler),
            )
            .route(
                "/events",
                get(async move || -> Result<Sse<_>, ServerError> {
                    // A `Stream` that repeats an event every second
                    let rx = tx.subscribe();
                    let receiver_stream = BroadcastStream::new(rx);
                    let stream = receiver_stream.map(|result| {
                        result.map(|data| Event::default().data(data))
                    });
/*                    let stream = stream::repeat_with(|| Event::default().data("hi!"))
                        .map(Ok::<Event, Infallible>)
                        .throttle(Duration::from_secs(1));*/

                    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
                }),
            );

        task::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(1000));
            let mut i = 0;
            loop {
                interval.tick().await;
                println!("tick {i}");
                tx2.send(format!("tick {i}")).unwrap();
                i += 1;
            }
        });
        // run our app with hyper, listening globally on port 3333
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

/*
#[axum::debug_handler]
async fn index_handler() -> Result<Response<String>, ServerError> {
    Ok(Response::builder().status(StatusCode::OK).body("Hi, World!".to_string())?)
}

 */

async fn sse_handler() -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ServerError> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}