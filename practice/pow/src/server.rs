use anyhow::Result;
use futures::Stream;
use pow::pb::pow_builder_server::*;
use pow::pb::*;
use std::pin::Pin;
use std::sync::Arc;
use std::{collections::HashMap, thread};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

const CHANNEL_SIZE: usize = 8;

#[derive(Debug, Default)]
struct Shared {
    clients: HashMap<String, mpsc::Sender<Result<BlockHash, Status>>>,
}

impl Shared {
    async fn broadcast(&self, msg: Option<BlockHash>) {
        let msg = msg.ok_or(Status::resource_exhausted(
            "Failed to find a suitabe hash".to_string(),
        ));
        for (name, tx) in &self.clients {
            match tx.send(msg.clone()).await {
                Ok(_) => (),
                Err(err) => {
                    println!(
                        "Broadcast error to {} with {:?}, Error: {:?}",
                        name, msg, err
                    )
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PowService {
    //send block to  pow engine
    tx: mpsc::Sender<Block>,
    shared: Arc<RwLock<Shared>>,
}

#[tonic::async_trait]
impl PowBuilder for PowService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockHash, Status>> + Send + Sync>>;

    async fn subscribe(
        &self,
        request: Request<ClientInfo>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let name = request.into_inner().name;

        let (tx, rx) = mpsc::channel(CHANNEL_SIZE);
        self.shared.write().await.clients.insert(name, tx);

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn submit(&self, request: Request<Block>) -> Result<Response<BlockStatus>, Status> {
        let block = request.into_inner();
        match self.tx.send(block.clone()).await {
            Ok(()) => Ok(Response::new(BlockStatus { code: 0 })),
            Err(err) => {
                println!(
                    "Failed to submit {:?} to PoW engine, error : {:?}",
                    block, err
                );
                Ok(Response::new(BlockStatus { code: 500 }))
            }
        }
    }
}

impl PowService {
    pub fn new(tx: mpsc::Sender<Block>, mut rx: mpsc::Receiver<Option<BlockHash>>) -> Self {
        let server = Self {
            tx,
            shared: Arc::new(RwLock::new(Shared::default())),
        };

        let shared = server.shared.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                shared.read().await.broadcast(msg).await;
            }
        });

        server
    }
}

async fn start_server(addr: &str) -> Result<()> {
    let addr = addr.parse().unwrap();

    //grpc -> PoW
    let (tx1, mut rx1) = mpsc::channel(CHANNEL_SIZE);

    //PoW -> grpc
    let (tx2, rx2) = mpsc::channel(CHANNEL_SIZE);

    thread::spawn(move || {
        while let Some(block) = rx1.blocking_recv() {
            let result = pow::powork::pow(block);
            tx2.blocking_send(result).unwrap();
        }
    });

    let svc = PowService::new(tx1, rx2);

    Server::builder()
        .add_service(PowBuilderServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8888";
    println!("Listen on {:?}", addr);
    start_server(addr).await?;

    Ok(())
}
