
//src/actors/scheduler.rs

use actix::prelude::*;
use chrono::Utc;
use crate::models::Job;
use crate::messages::*;
use crate::utils::exponential_backoff;
use std::collections::VecDeque;

pub struct SchedulerActor {
    pub ready: VecDeque<Job>, // jobs ready to run now
}

impl SchedulerActor {
    pub fn new() -> Self {
        SchedulerActor {
            ready: VecDeque::new(),
        }
    }
}

impl Actor for SchedulerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Periodically check for jobs that are due
        ctx.run_interval(std::time::Duration::from_secs(1), |act, _ctx| {
            let now = Utc::now();
            act.ready.retain(|job| job.scheduled_for <= now);
        });
    }
}

impl Handler<Enqueue> for SchedulerActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: Enqueue, _: &mut Context<Self>) -> Self::Result {
        self.ready.push_back(msg.0);
        Ok(())
    }
    
    // fn handle(&mut self, msg: Enqueue, _: &mut Context<Self>) {
    //     let job = msg.0;
    //     self.ready.push_back(job);
    // }
}

impl Handler<Dequeue> for SchedulerActor {
    type Result = Option<Job>;

    fn handle(&mut self, _: Dequeue, _: &mut Context<Self>) -> Self::Result {
        self.ready.pop_front()
    }
}
impl Handler<Nack> for SchedulerActor {
    type Result = ();

    fn handle(&mut self, msg: Nack, _: &mut Context<Self>) {
        // Find the job in the ready queue
        if let Some(pos) = self.ready.iter().position(|j| j.id == msg.0) {
            let mut job = self.ready.remove(pos).unwrap();
            job.retries += 1;

            if job.retries <= job.max_retries {
                // compute new backoff
                let delay = exponential_backoff(job.retries);
                job.scheduled_for = Utc::now() + delay;
                println!(
                    "Rescheduled job {} with {}s delay (retry #{})",
                    job.id,
                    delay.num_seconds(),
                    job.retries
                );
                self.ready.push_back(job);
            } else {
                println!("Job {} exceeded max retries", job.id);
            }
        }
    }
}
