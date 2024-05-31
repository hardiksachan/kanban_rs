use super::{domain, ports, Result};
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

impl From<domain::Ticket> for TicketResponse {
    fn from(value: domain::Ticket) -> Self {
        TicketResponse {
            id: value.ticket_id().get(),
            owner_id: value.owner_id().get(),
            title: value.title().get(),
        }
    }
}

impl From<domain::Ticket> for CreateTicketResponse {
    fn from(ticket: domain::Ticket) -> Self {
        Self {
            id: ticket.ticket_id().get(),
        }
    }
}

pub struct Ticket<S>
where
    S: ports::TicketStore,
{
    store: S,
}

impl<S> Clone for Ticket<S>
where
    S: ports::TicketStore,
{
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

impl<S> Ticket<S>
where
    S: ports::TicketStore,
{
    pub fn with_store(store: S) -> Self
    where
        S: ports::TicketStore,
    {
        Self { store }
    }
}

impl<S> Ticket<S>
where
    S: ports::TicketStore,
{
    #[instrument(skip(self))]
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: CreateTicketRequest,
    ) -> Result<CreateTicketResponse> {
        let ticket = domain::Ticket::new(ctx.user_id().into(), ticket_fc.title.into());

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
            .into_iter()
            .map(|t| t.into())
            .collect();

        info!(count = tickets.len(), "tickets listed");

        Ok(tickets)
    }
}
