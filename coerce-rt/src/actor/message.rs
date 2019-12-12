use crate::actor::Actor;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub trait Message {
    type Result;
}

pub type HandleFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

#[async_trait]
pub trait Handler<Msg: Message + Send + Sync>
where
    Msg::Result: 'static + Send + Sync,
{
    async fn handle(&mut self, message: Msg) -> Msg::Result;
}

#[derive(Debug)]
pub enum MessageResult<T> {
    Ok(T),
    Error,
}

pub struct ActorMessage<A: Actor, M: Message>
where
    A: Handler<M> + Send + Sync,
    M: Send + Sync,
    M::Result: 'static + Send + Sync,
{
    msg: Option<M>,
    sender: Option<tokio::sync::oneshot::Sender<M::Result>>,
    _a: PhantomData<A>,
}

#[async_trait]
pub trait ActorMessageHandler<A>: Sync + Send
where
    A: Actor + Sync + Send,
{
    async fn handle(&mut self, actor: &mut A) -> ();
}

#[async_trait]
impl<A: 'static + Actor, M: 'static + Message> ActorMessageHandler<A> for ActorMessage<A, M>
where
    A: Handler<M> + Send + Sync,
    M: Send + Sync,
    M::Result: Send + Sync,
{
    async fn handle(&mut self, actor: &mut A) -> () {
        self.handle_msg(actor).await;
    }
}

impl<A: 'static + Actor, M: 'static + Message> ActorMessage<A, M>
where
    A: Handler<M> + Send + Sync,
    M: Send + Sync,
    M::Result: Send + Sync,
{
    pub fn new(msg: M, sender: tokio::sync::oneshot::Sender<M::Result>) -> ActorMessage<A, M> {
        ActorMessage {
            msg: Some(msg),
            sender: Some(sender),
            _a: PhantomData,
        }
    }

    pub async fn handle_msg(&mut self, actor: &mut A) {
        let sender = self.sender.take();
        let msg = self.msg.take();

        sender.unwrap().send(actor.handle(msg.unwrap()).await);
    }
}