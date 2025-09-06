#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:5000")?;
    let response = client
        .do_post(
            "/api/auth/login",
            json!({
                "username": "admin@gmail.com",
                "password": "admin123"
            })
        )
        .await?;

    response.print().await?;
    Ok(())
}
