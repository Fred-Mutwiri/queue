

//src/actors/consumer.rs

use actix::prelude::*;
use actix::fut::wrap_future;

use crate::messages::*;
// use crate::models::Job;

pub struct ConsumerActor {
    pub scheduler: Addr<crate::actors::scheduler::SchedulerActor>,
}

impl Actor for ConsumerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Pull a job every 2 seconds
        ctx.run_interval(std::time::Duration::from_secs(2), |act, ctx| {
            let sched = act.scheduler.clone();
            ctx.spawn(
                wrap_future(async move {
                    if let Some(job) = sched.send(Dequeue).await.unwrap() {
                        println!("Consumer got job: {:?}", job);
                        // Later: forward job to dispatcher
                    }
                })
            );
        });
    }
}
