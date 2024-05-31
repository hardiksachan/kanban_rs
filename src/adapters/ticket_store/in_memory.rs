use crate::core::ports;
use crate::Result;
use crate::{core::domain, ctx::Ctx};
use axum::async_trait;
use std::sync::{Arc, Mutex};
use tracing::{info, instrument};

#[derive(Clone)]
pub struct InMemory {
    tickets_store: Arc<Mutex<Vec<Option<domain::Ticket>>>>,
}

impl InMemory {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

#[async_trait]
impl ports::TicketStore for InMemory {
    #[instrument(skip(self))]
    async fn save_ticket(&self, _ctx: Ctx, ticket: domain::Ticket) -> Result<()> {
        let mut store = self.tickets_store.lock().unwrap();

        store.push(Some(ticket.clone()));

        info!(?ticket, "ticket added");

        Ok(())
    }

    #[instrument(skip(self))]
    async fn list_all_tickets(&self, _ctx: Ctx) -> Result<Vec<domain::Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect::<Vec<_>>();

        info!(count = tickets.len(), "tickets listed");

        Ok(tickets)
    }

    //#[instrument(skip(self))]
    //pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
    //    let mut store = self.tickets_store.lock().unwrap();
    //
    //    let ticket = store.get_mut(id as usize).and_then(|t| t.take());
    //
    //    info!(?ticket, "ticket deleted");
    //
    //    ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    //}
}
