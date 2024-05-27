use crate::ctx::Ctx;
use crate::Result;
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, response::IntoResponse, Json};
use tracing::instrument;

use crate::model::{ModelController, TicketForCreate};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

#[instrument(skip(mc))]
async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> impl IntoResponse {
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Result::Ok(Json(ticket))
}

#[instrument(skip(mc))]
async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> impl IntoResponse {
    let tickets = mc.list_tickets(ctx).await?;

    Result::Ok(Json(tickets))
}

#[instrument(skip(mc))]
async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let ticket = mc.delete_ticket(ctx, id).await?;

    Result::Ok(Json(ticket))
}
