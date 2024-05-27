use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tracing::{info, instrument};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub uid: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    #[instrument(skip(self))]
    pub async fn create_ticket(&self, ctx: Ctx, tickec_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            uid: ctx.user_id(),
            title: tickec_fc.title,
        };
        store.push(Some(ticket.clone()));

        info!(?ticket, "ticket added");

        Ok(ticket)
    }

    #[instrument(skip(self))]
    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets: Vec<Ticket> = store.iter().filter_map(|t| t.clone()).collect();

        info!(count = tickets.len(), "tickets listed");

        Ok(tickets)
    }

    #[instrument(skip(self))]
    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        info!(?ticket, "ticket deleted");

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
