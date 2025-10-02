use actix::prelude::*;
use crate::messages::*;
use crate::models::Job;
use crate::actors::storage::StorageActor;

pub struct QueueManager {
    pub storage: Addr<StorageActor>,
}

impl Actor for QueueManager {
    type Context = Context<Self>;
}

impl Handler<Enqueue> for QueueManager {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: Enqueue, _: &mut Context<Self>) -> Self::Result {
        let storage = self.storage.clone();
        Box::pin(async move {
            storage.send(msg).await.unwrap()?;
            Ok(())
        })
    }
}

impl Handler<Dequeue> for QueueManager {
    type Result = ResponseFuture<Option<Job>>;

    fn handle(&mut self, msg: Dequeue, _: &mut Context<Self>) -> Self::Result {
        let storage = self.storage.clone();
        Box::pin(async move { storage.send(msg).await.unwrap() })
    }
}
