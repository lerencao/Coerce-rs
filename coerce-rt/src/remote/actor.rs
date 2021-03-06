use crate::actor::context::{ActorContext, ActorHandlerContext};
use crate::actor::message::{Handler, Message};
use crate::actor::{Actor, ActorRef};
use crate::remote::handler::{RemoteActorMessageMarker, RemoteMessageHandler};
use std::any::TypeId;
use std::collections::HashMap;

pub(crate) type BoxedHandler = Box<dyn RemoteMessageHandler + Send + Sync>;

pub struct RemoteRegistry {}

pub struct RemoteHandler {
    handler_types: HashMap<TypeId, String>,
    handlers: HashMap<String, BoxedHandler>,
}

impl Actor for RemoteRegistry {}

impl Actor for RemoteHandler {}

impl RemoteRegistry {
    pub async fn new(ctx: &mut ActorContext) -> ActorRef<RemoteRegistry> {
        ctx.new_actor(RemoteRegistry {}).await.unwrap()
    }
}

impl RemoteHandler {
    pub async fn new(
        ctx: &mut ActorContext,
        handlers: HashMap<String, BoxedHandler>,
        handler_types: HashMap<TypeId, String>,
    ) -> ActorRef<RemoteHandler> {
        ctx.new_actor(RemoteHandler {
            handler_types,
            handlers,
        })
        .await
        .unwrap()
    }
}

pub struct GetHandler(pub String);

pub struct HandlerName<A: Actor, M: Message>
where
    A: 'static + Send + Sync,
    M: 'static + Send + Sync,
    M::Result: 'static + Sync + Send,
{
    marker: RemoteActorMessageMarker<A, M>,
}

impl<A: Actor, M: Message> HandlerName<A, M>
where
    A: 'static + Send + Sync,
    M: 'static + Send + Sync,
    M::Result: 'static + Send + Sync,
{
    pub fn new() -> HandlerName<A, M> {
        HandlerName {
            marker: RemoteActorMessageMarker::new(),
        }
    }
}

impl<A: Actor, M: Message> Message for HandlerName<A, M>
where
    A: 'static + Send + Sync,
    M: 'static + Send + Sync,
    M::Result: 'static + Send + Sync,
{
    type Result = Option<String>;
}

impl Message for GetHandler {
    type Result = Option<BoxedHandler>;
}

#[async_trait]
impl Handler<GetHandler> for RemoteHandler {
    async fn handle(
        &mut self,
        message: GetHandler,
        _ctx: &mut ActorHandlerContext,
    ) -> Option<BoxedHandler> {
        match self.handlers.get(&message.0) {
            Some(handler) => Some(handler.new_boxed()),
            None => None,
        }
    }
}

#[async_trait]
impl<A: Actor, M: Message> Handler<HandlerName<A, M>> for RemoteHandler
where
    A: 'static + Send + Sync,
    M: 'static + Send + Sync,
    M::Result: 'static + Sync + Send,
{
    async fn handle(
        &mut self,
        message: HandlerName<A, M>,
        _ctx: &mut ActorHandlerContext,
    ) -> Option<String> {
        match self.handler_types.get(&message.marker.id()) {
            Some(name) => Some(name.clone()),
            None => None,
        }
    }
}
