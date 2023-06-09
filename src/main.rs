#![feature(slice_group_by)]

use axum::{extract::Path, response::Html, routing::get, Json, Router};
use std::{env, net::SocketAddr};
use types::{FactionPercents, World, Zone};
use zones::get_zone_states;

mod alert_types;
mod alerts;
mod misc;
mod types;
mod zones;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/:world", get(get_world))
        .route("/", get(root));

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        env::var("PORT")
            .unwrap_or("8076".to_string())
            .parse()
            .unwrap(),
    ));
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(include_str!("./html/index.html"))
}

#[axum::debug_handler]
pub async fn get_world(Path(world): Path<i32>) -> Json<World> {
    Json(World {
        id: world,
        zones: get_zone_states(world).await.unwrap(),
    })
}
