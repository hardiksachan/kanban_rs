use crate::core;
use crate::core::ports::TicketStore;
use crate::ctx::Ctx;
use crate::Result;
//use axum::extract::Path;
use axum::routing::post;
use axum::Router;
use axum::{extract::State, response::IntoResponse, Json};
use tracing::instrument;

pub fn routes<S>(svc: core::services::Ticket<S>) -> Router
where
    S: TicketStore + 'static,
{
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        //.route("/tickets/:id", delete(delete_ticket))
        .with_state(svc)
}

#[instrument(skip(svc))]
async fn create_ticket<S>(
    State(svc): State<core::services::Ticket<S>>,
    ctx: Ctx,
    Json(create_ticket_req): Json<core::services::CreateTicketRequest>,
) -> impl IntoResponse
where
    S: core::ports::TicketStore,
{
    let ticket = svc.create_ticket(ctx, create_ticket_req).await?;

    Result::Ok(Json(ticket))
}

#[instrument(skip(svc))]
async fn list_tickets<S>(
    State(svc): State<core::services::Ticket<S>>,
    ctx: Ctx,
) -> impl IntoResponse
where
    S: core::ports::TicketStore,
{
    let tickets = svc.list_tickets(ctx).await?;

    Result::Ok(Json(tickets))
}
