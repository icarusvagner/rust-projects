#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello?name=vladiere").await?.print().await?;
    hc.do_get("/name/vladiere").await?.print().await?;

    let req_body = hc.do_post("/api/login", json!({
        "username" : "test1",
        "password" : "pass1",
    }));
    req_body.await?.print().await?;

    let req_create_ticket = hc.do_post("/api/tickets", json!({
        "title" : "Tickets 1A"
    }));
    req_create_ticket.await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;
    hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
