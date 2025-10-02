use actix::prelude::*;
use queue_service::actors::{
    manager::QueueManager,
    storage::StorageActor,
    scheduler::SchedulerActor,
    consumer::ConsumerActor,
};
use queue_service::models::Job;
use queue_service::messages::Enqueue;
use queue_service::utils::now;
use uuid::Uuid;

#[actix_rt::main]
async fn main() {
    let storage = StorageActor::new().start();
    let scheduler = SchedulerActor::new().start();
    let manager = QueueManager { storage: storage.clone() }.start();
    let _consumer = ConsumerActor { scheduler: scheduler.clone() }.start();

    // Add a job to the queue
    let job = Job {
        id: Uuid::new_v4(),
        payload: "Send email to bob@example.com".to_string(),
        retries: 0,
        max_retries: 3,
        scheduled_for: now(),
    };

    manager.send(Enqueue(job)).await.unwrap().unwrap();
    println!("Job enqueued!");
}
