use axum::async_trait;

use super::Result;

#[async_trait]
pub trait TicketStore: Clone + Send + Sync + 'static {
    async fn save_ticket(&self, ctx: super::ctx::Ctx, ticket: super::domain::Ticket) -> Result<()>;
    async fn list_all_tickets(
        &self,
        ctx: super::ctx::Ctx,
    ) -> Result<impl Iterator<Item = super::domain::Ticket>>;
    async fn delete_ticket(
        &self,
        ctx: super::ctx::Ctx,
        ticket: super::domain::Ticket,
    ) -> Result<()>;
}
