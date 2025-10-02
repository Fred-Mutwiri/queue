# queue
prototype of a highly performant in-memory queue that inserts data and pops it and a retry mechanism 

it uses actors model to manage and store the jobs in memory, through a dashmap. this way any job enque is  passed through a managerActor that stores it through a StorageActor and schedules it into the Queue and retrieves it through the ConsumerActor. the actors are concurrent and highly effective as they manage the coonnection and resource sharing highly reducing the chance of a deadlock occurrence. 

this is a basic prototype of a highly effective message_broker like RabbitMQ or Kafka which are superior Pub/Sub used to handle Millions of requests/sec.
