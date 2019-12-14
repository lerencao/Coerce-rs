use crate::util::{
    GetStatusRequest, GetStatusResponse, SetStatusRequest, SetStatusResponse, TestActor,
    TestActorStatus,
};
use coerce_rt::actor::context::{ActorContext, ActorHandlerContext};
use coerce_rt::actor::message::{Handler, Message, MessageResult};
use coerce_rt::actor::Actor;

#[macro_use]
extern crate async_trait;

pub mod util;

#[async_trait]
impl Handler<GetStatusRequest> for TestActor {
    async fn handle(
        &mut self,
        message: GetStatusRequest,
        _ctx: &mut ActorHandlerContext,
    ) -> GetStatusResponse {
        match self.status {
            Some(TestActorStatus::Active) => GetStatusResponse::Ok(TestActorStatus::Active),
            Some(TestActorStatus::Inactive) => GetStatusResponse::Ok(TestActorStatus::Inactive),
            _ => GetStatusResponse::None,
        }
    }
}

#[async_trait]
impl Handler<SetStatusRequest> for TestActor {
    async fn handle(
        &mut self,
        message: SetStatusRequest,
        _ctx: &mut ActorHandlerContext,
    ) -> SetStatusResponse {
        self.status = Some(message.0);

        SetStatusResponse::Ok
    }
}

#[async_trait]
impl Actor for TestActor {}

#[tokio::test]
pub async fn test_actor_req_res() {
    let ctx = ActorContext::new();
    let mut actor_ref = ctx.lock().unwrap().new_actor(TestActor::new());

    let response = actor_ref.send(GetStatusRequest {}).await;

    assert_eq!(response, Ok(GetStatusResponse::None));
}

#[tokio::test]
pub async fn test_actor_req_res_mutation() {
    let ctx = ActorContext::new();
    let mut actor_ref = ctx.lock().unwrap().new_actor(TestActor::new());

    let initial_status = actor_ref.send(GetStatusRequest {}).await;
    let _ = actor_ref
        .send(SetStatusRequest(TestActorStatus::Active))
        .await;
    let current_status = actor_ref.send(GetStatusRequest {}).await;

    assert_eq!(initial_status, Ok(GetStatusResponse::None));
    assert_eq!(
        current_status,
        Ok(GetStatusResponse::Ok(TestActorStatus::Active))
    );
}