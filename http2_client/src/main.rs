use http::{Request, StatusCode, Uri};
use http_body_util::Empty;
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use std::{
    error::Error,
    time::{Duration, Instant},
};

use anyhow::Result;

struct RequestCtx {
    num: usize,
    start: std::time::Instant,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let connector = HttpConnector::new();
    let client: Client<HttpConnector, Empty<bytes::Bytes>> =
        Client::builder(hyper_util::rt::TokioExecutor::new())
            .http2_only(true)
            .build(connector);

    // Initial request to be sure that http2 settings are sent
    let _ = perform_req(
        client.clone(),
        RequestCtx {
            num: 0,
            start: Instant::now(),
        },
        None,
    )
    .await;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    tracing::info!("Start sending requests");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<(usize, Duration, StatusCode)>(10);

    for i in 0..10 {
        let context = RequestCtx {
            num: i,
            start: std::time::Instant::now(),
        };
        let _ = perform_req(client.clone(), context, Some(tx.clone())).await;
    }

    for _ in 0..10 {
        let (num, duration, status) = rx.recv().await.unwrap();
        tracing::info!(
            "Request {} took {:?} and returned {:?}",
            num,
            duration,
            status
        );
    }

    Ok(())
}

async fn perform_req(
    client: Client<HttpConnector, Empty<bytes::Bytes>>,
    RequestCtx { num, start }: RequestCtx,
    tx: Option<tokio::sync::mpsc::Sender<(usize, Duration, StatusCode)>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url: Uri = format!("http://127.0.0.1:8080/{num}").parse().unwrap();
    let req = Request::builder()
        .uri(url.clone())
        .body(Empty::<bytes::Bytes>::new())?;
    let client = client.clone();

    tokio::spawn(async move {
        tracing::debug!("Sending request {num}: {:?}", req);
        let resp = client.request(req).await?;

        if let Some(tx) = tx {
            tx.send((num, start.elapsed(), resp.status()))
                .await
                .unwrap();
        };

        Ok::<(), anyhow::Error>(()) // <- note the explicit type annotation here
    });

    Ok(())
}
