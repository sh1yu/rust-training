use pow::pb::{pow_builder_client::PowBuilderClient, Block, ClientInfo};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "http://localhost:8888";
    let mut client = PowBuilderClient::connect(addr).await?;

    let mut stream = client
        .subscribe(ClientInfo {
            name: "client1".into(),
        })
        .await?
        .into_inner();

    let res = client
        .submit(Block {
            data: b"Hello world".to_vec(),
            ..Default::default()
        })
        .await?
        .into_inner();

    println!("Submitted: {:?}", res);

    while let Some(result) = stream.message().await? {
        println!(
            "Result - id: {}, hash: {}, nonce: {}",
            hex::encode(result.id),
            hex::encode(result.hash),
            result.nonce
        );
    }

    Ok(())
}
