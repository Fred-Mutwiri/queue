use actix::prelude::*;
use crate::models::Job;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Enqueue(pub Job);

#[derive(Message)]
#[rtype(result = "Option<Job>")]
pub struct Dequeue;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ack(pub Uuid);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Nack(pub Uuid);
