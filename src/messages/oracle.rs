use actix::{Actor, ActorContext, ActorState, AsyncContext, Context, Message, Recipient, Running, System};
use actix::Handler;
use rand;
use rand::Rng;
use std::time::Duration;

pub struct RandomNumber(usize);

impl Message for RandomNumber {
    type Result = ();
}

pub struct RandomOracle<R: Rng> {
    subscribers: Vec<Recipient<RandomNumber>>,
    period: Duration,
    oracle: R,
}

impl RandomOracle<rand::OsRng> {
    pub fn new(dur: Duration) -> RandomOracle<rand::OsRng> {
        let rng = rand::OsRng::new().unwrap();
        RandomOracle {
            subscribers: Vec::new(),
            period: dur,
            oracle: rng,
        }
    }

    fn get_next(&mut self) -> usize {
        self.oracle.gen_range(0, 20)
    }

    fn broadcast(&mut self, ctx: &mut Context<Self>) {
        if ctx.state() == ActorState::Running {
            let val = self.get_next();
            self.subscribers.retain(|sub| {
                sub.do_send(RandomNumber(val)).is_ok()
            });
            println!("Oracle sent new value: {} to {} subs", val, self.subscribers.len());
            ctx.run_later(self.period, |act, ctx| {
                act.broadcast(ctx);
            });
            if self.subscribers.len() == 0 {
                println!("No more subscriber left :(");
                ctx.stop();
                System::current().stop();
            }
        } else {
            println!("Oracle has sent its last value");
        }
    }

    pub fn add_listener(&mut self, listener: Recipient<RandomNumber>) {
        self.subscribers.push(listener);
    }
}

impl Actor for RandomOracle<rand::OsRng> {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Starting Oracle");
        self.broadcast(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Oracle is shutting down");
        Running::Stop
    }
}


pub struct OracleListener {
    pub name: String,
    pub secret: usize,
}

impl Actor for OracleListener {
    type Context = Context<Self>;
}

impl Handler<RandomNumber> for OracleListener {
    type Result = ();

    fn handle(&mut self, msg: RandomNumber, ctx: &mut Self::Context) -> <Self as Handler<RandomNumber>>::Result {
//        println!("{} received: {}", self.name, msg.0);
        if msg.0 == self.secret {
            println!("You guessed {}'s number", self.name);
            ctx.stop();
        }
    }
}
