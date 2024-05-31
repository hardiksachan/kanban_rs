use super::Result;

pub trait TicketStore: Clone + Send + Sync {
    fn save_ticket(
        &self,
        ctx: super::ctx::Ctx,
        ticket: super::domain::Ticket,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn list_all_tickets(
        &self,
        ctx: super::ctx::Ctx,
    ) -> impl std::future::Future<Output = Result<impl Iterator<Item = super::domain::Ticket>>> + Send;
    async fn delete_ticket(
        &self,
        ctx: super::ctx::Ctx,
        ticket: super::domain::Ticket,
    ) -> Result<()>;
}
