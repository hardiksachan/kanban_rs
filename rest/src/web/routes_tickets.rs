use super::extract::Ctx;
use crate::Result;
use axum::extract::Path;
use axum::routing::{delete, get, post};
use axum::Router;
use axum::{extract::State, response::IntoResponse, Json};
use tickets::core::services::DeleteTicketRequest;
use tickets::core::{self, ports};
use tracing::instrument;

pub fn routes<S>() -> Router<core::services::Ticket<S>>
where
    S: ports::TicketStore + 'static,
{
    Router::new()
        .route("/tickets", post(create_ticket::<S>))
        .route("/tickets", get(list_tickets::<S>))
        .route("/tickets/:ticket_id", delete(delete_ticket::<S>))
}

#[instrument(skip(svc))]
async fn create_ticket<S>(
    State(svc): State<core::services::Ticket<S>>,
    Ctx(ctx): Ctx,
    Json(req): Json<core::services::CreateTicketRequest>,
) -> impl IntoResponse
where
    S: ports::TicketStore,
{
    let ticket = svc.create_ticket(ctx, req).await?;

    Result::Ok(Json(ticket))
}

#[instrument(skip(svc))]
async fn list_tickets<S>(
    State(svc): State<core::services::Ticket<S>>,
    Ctx(ctx): Ctx,
) -> impl IntoResponse
where
    S: ports::TicketStore,
{
    let tickets = svc.list_tickets(ctx).await?;

    Result::Ok(Json(tickets))
}

#[instrument(skip(svc))]
async fn delete_ticket<S>(
    State(svc): State<core::services::Ticket<S>>,
    Ctx(ctx): Ctx,
    Path(req): Path<DeleteTicketRequest>,
) -> impl IntoResponse
where
    S: ports::TicketStore,
{
    let ticket = svc.delete_ticket(ctx, req).await?;

    Result::Ok(Json(ticket))
}
