use crate::Result;
use axum::extract::Path;
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, response::IntoResponse, Json};

use crate::model::{ModelController, TicketForCreate};

pub fn routes(
    mc: ModelController,
) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> impl IntoResponse {
    println!("->> {:<12} - create-ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Result::Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> impl IntoResponse {
    println!("->> {:<12} - list-tickets", "HANDLER");

    let tickets = mc.list_tickets().await?;

    Result::Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    println!("->> {:<12} - delete-ticket", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Result::Ok(Json(ticket))
}
