use axum::async_trait;

use super::Result;
use crate::core::domain;
use crate::ctx;

#[async_trait]
pub trait TicketStore: Clone + Send + Sync {
    async fn save_ticket(&self, ctx: ctx::Ctx, ticket: domain::Ticket) -> Result<()>;
    async fn list_all_tickets(&self, ctx: ctx::Ctx) -> Result<Vec<domain::Ticket>>;
    //async fn delete_ticket(
    //    &self,
    //    ctx: super::ctx::Ctx,
    //    ticket: super::domain::Ticket,
    //) -> Result<()>;
}
