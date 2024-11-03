use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000/?name=John")?;
    hc.do_get("/").await?.print().await?;
    Ok(())
}