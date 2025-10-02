//src/actors/storage.rs

use actix::prelude::*;
use crate::messages::*;
use crate::models::Job;
use dashmap::DashMap;
use uuid::Uuid;
use std::sync::Arc;

pub struct StorageActor {
    pub jobs: Arc<DashMap<Uuid, Job>>,
}

impl StorageActor {
    pub fn new() -> Self {
        StorageActor {
            jobs: Arc::new(DashMap::new()),
        }
    }
}

impl Actor for StorageActor {
    type Context = Context<Self>;
}

impl Handler<Enqueue> for StorageActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: Enqueue, _: &mut Context<Self>) -> Self::Result {
        self.jobs.insert(msg.0.id, msg.0);
        Ok(())
    }
}

impl Handler<Dequeue> for StorageActor {
    type Result = Option<Job>;

    fn handle(&mut self, _: Dequeue, _: &mut Context<Self>) -> Self::Result {
        self.jobs.iter().next().map(|entry| entry.value().clone())
    }
}

impl Handler<Ack> for StorageActor {
    type Result = ();

    fn handle(&mut self, msg: Ack, _: &mut Context<Self>) {
        self.jobs.remove(&msg.0);
    }
}

impl Handler<Nack> for StorageActor {
    type Result = ();

    fn handle(&mut self, msg: Nack, _: &mut Context<Self>) {
        if let Some(mut job) = self.jobs.get_mut(&msg.0) {
            job.retries += 1;
        }
    }
}
