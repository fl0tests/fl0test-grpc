use std::sync::{Arc, RwLock};

pub mod canary {
    tonic::include_proto!("canary");
}
use canary::canary_server::Canary;
use canary::{HealtcheckResponse, HealthcheckRequest, StatsRequest, StatsResponse};
use nanoid::nanoid;
use time::format_description::well_known::Rfc3339;
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct GrpcServer {
    channel_capacity: usize,
    counter: Arc<RwLock<u32>>,
}

impl GrpcServer {
    pub fn new(channel_capacity: usize) -> Self {
        Self {
            channel_capacity,
            counter: Arc::new(RwLock::new(0)),
        }
    }
}

#[tonic::async_trait]
impl Canary for GrpcServer {
    async fn healthcheck(
        &self,
        request: Request<HealthcheckRequest>,
    ) -> Result<Response<HealtcheckResponse>, Status> {
        let peer_addr = request.remote_addr();
        let metadata = request.metadata();
        tracing::info!(?peer_addr, metadata=?metadata, "received healthcheck request");
        let mut served_count = 0;
        if let Ok(mut guard) = self.counter.write() {
            *guard += 1;
            served_count = *guard;
        };
        let response = HealtcheckResponse {
            message: "Service is healthy!".to_string(),
            timestamp: time::OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .unwrap_or_default(),
            served: served_count,
        };
        Ok(Response::new(response))
    }

    type StatsStream = ReceiverStream<Result<StatsResponse, Status>>;

    async fn stats(
        &self,
        request: Request<StatsRequest>,
    ) -> Result<Response<Self::StatsStream>, Status> {
        if let Ok(mut guard) = self.counter.write() {
            *guard += 1;
        };
        let peer_addr = request.remote_addr();
        let (meta, _ext, request) = request.into_parts();
        let max_messages = request.max_messages;
        tracing::info!(?peer_addr, metadata=?meta, "received stats stream request");
        let (tx, rx) = mpsc::channel(self.channel_capacity);
        let spawn_res = tokio::spawn(async move { stats_generator(tx, max_messages).await });
        tracing::info!(?spawn_res);
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

async fn stats_generator(tx: Sender<Result<StatsResponse, Status>>, max_messages: u32) {
    tracing::info!(message_count = max_messages, "Spawned stats generator");
    for msg_count in 1..=max_messages {
        let msg = StatsResponse {
            message: "Generated stats message".to_string(),
            timestamp: time::OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .unwrap_or_default(),
            message_count: msg_count,
            message_id: nanoid!(12),
        };
        let _result = tx.send(Ok(msg)).await;
    }
    tracing::info!(message_count = max_messages, "Finished stats generation");
}
