use super::Result;
use crate::ctx::Ctx;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTicketResponse {
    pub id: u64,
}

#[derive(Serialize)]
pub struct TicketResponse {
    id: u64,
    owner_id: u64,
    title: String,
}

impl From<super::domain::Ticket> for TicketResponse {
    fn from(value: super::domain::Ticket) -> Self {
        TicketResponse {
            id: value.ticket_id().get(),
            owner_id: value.owner_id().get(),
            title: value.title().get(),
        }
    }
}

impl From<super::domain::Ticket> for CreateTicketResponse {
    fn from(ticket: super::domain::Ticket) -> Self {
        Self {
            id: ticket.ticket_id().get(),
        }
    }
}

#[derive(Clone)]
pub struct Ticket<S>
where
    S: Clone,
{
    store: S,
}

impl<S> Ticket<S>
where
    S: super::ports::TicketStore,
{
    #[instrument(skip(self))]
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: CreateTicketRequest,
    ) -> Result<CreateTicketResponse> {
        let ticket = super::domain::Ticket::new(ctx.user_id().into(), ticket_fc.title.into());

        self.store.save_ticket(ctx, ticket.clone()).await?;
        info!(?ticket, "ticket added");

        Ok(ticket.into())
    }

    #[instrument(skip(self))]
    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<TicketResponse>> {
        let tickets: Vec<TicketResponse> = self
            .store
            .list_all_tickets(ctx)
            .await?
            .map(|t| t.into())
            .collect();

        info!(count = tickets.len(), "tickets listed");

        Ok(tickets)
    }
}
