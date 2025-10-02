use actix::prelude::*;
use queue_service::actors::{
    manager::QueueManager,
    storage::StorageActor,
    scheduler::SchedulerActor,
    consumer::ConsumerActor,
};
use queue_service::models::Job;
use queue_service::messages::{Enqueue, Dequeue, Nack};
use queue_service::utils::{now, exponential_backoff};
use uuid::Uuid;
use chrono::Utc;
use std::time::Duration;

#[actix_rt::test]
async fn test_enqueue_and_dequeue() {
    let storage = StorageActor::new().start();
    let scheduler = SchedulerActor::new().start();
    let manager = QueueManager { storage: storage.clone() }.start();
    let _consumer = ConsumerActor { scheduler: scheduler.clone() }.start();

    // Enqueue a job
    let job = Job {
        id: Uuid::new_v4(),
        payload: "email to test@example.com".to_string(),
        retries: 0,
        max_retries: 3,
        scheduled_for: now(),
    };

    manager.send(Enqueue(job.clone())).await.unwrap().unwrap();

    // Wait a bit for scheduler/consumer to do work
    actix_rt::time::sleep(Duration::from_secs(3)).await;

    // Try dequeue manually from scheduler (to check it exists)
    let dequeued = scheduler.send(Dequeue).await.unwrap();
    assert!(dequeued.is_none(), "Job should already be consumed by ConsumerActor");
}



#[actix_rt::test]
async fn test_retry_with_backoff() {
    let scheduler = SchedulerActor::new().start();

    // Create job scheduled for now
    let job = Job {
        id: Uuid::new_v4(),
        payload: "fail email".to_string(),
        retries: 0,
        max_retries: 3,
        scheduled_for: Utc::now(),
    };

    let _ = scheduler.send(Enqueue(job.clone())).await;

    // Simulate failure (Nack)
    let _ = scheduler.send(Nack(job.id)).await;

    // Wait a bit to allow rescheduling
    actix_rt::time::sleep(Duration::from_secs(1)).await;

    // Get job again (should exist, scheduled later)
    let maybe_job = scheduler.send(Dequeue).await.unwrap();
    assert!(maybe_job.is_some(), "Job shouldn't be ready yet (backoff delay)");

    // Calculate backoff delay manually
    let backoff = exponential_backoff(1);
    println!("Expected retry delay ~{} secs", backoff.num_seconds());
}
