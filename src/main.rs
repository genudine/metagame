#![feature(slice_group_by)]

use alerts::get_alerts;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Json, Router,
};
use std::{env, net::SocketAddr};
use tokio::task::JoinSet;
use types::{World, Zone};
use zones::get_zone_states;

mod alert_types;
mod alerts;
mod misc;
mod types;
mod zones;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // .with_max_level(Level::DEBUG)
        .init();

    let db = sled::open("/tmp/metagame").expect("open");

    let app = Router::new()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/:world", get(get_one_world))
        .route("/all", get(get_all_worlds))
        .route("/", get(root))
        .with_state(db.clone());

    tokio::spawn(async move {
        loop {
            let mut set = JoinSet::new();
            for world in vec![1, 10, 13, 17, 19, 40, 1000, 2000] {
                set.spawn(get_world(db.clone(), world, true));
            }

            while let Some(_) = set.join_next().await {}

            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 3)).await;
        }
    });

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
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

pub async fn get_one_world(State(db): State<sled::Db>, Path(world): Path<i32>) -> Json<World> {
    Json(get_world(db, world, false).await)
}

pub async fn get_all_worlds(State(db): State<sled::Db>) -> Json<Vec<World>> {
    let mut set = JoinSet::new();
    let mut worlds = vec![World::default(); 8];

    for world in vec![1, 10, 13, 17, 19, 40, 1000, 2000] {
        set.spawn(get_world(db.clone(), world, false));
    }

    let mut i = 0;
    while let Some(response) = set.join_next().await {
        worlds[i] = response.unwrap_or_default();
        i += 1;
    }

    Json(worlds)
}

pub async fn get_world(db: sled::Db, world: i32, skip_cache: bool) -> World {
    if !skip_cache {
        match world_from_cache(db.clone(), world) {
            Ok(response) => return response,
            _ => {}
        }
    }

    let alerts = get_alerts(world).await.unwrap();
    let zones = get_zone_states(world).await.unwrap();

    let converged_zones: Vec<Zone> = zones
        .into_iter()
        .map(|zone| {
            let mut zone = zone;
            let alert = alerts.iter().find(|alert| alert.zone == zone.id);

            zone.alert = alert.cloned();

            zone
        })
        .collect();

    let response = World {
        id: world,
        zones: converged_zones,
        cached_at: chrono::Utc::now(),
    };

    world_to_cache(db, world, &response);

    response
}

fn world_from_cache(db: sled::Db, world: i32) -> Result<World, ()> {
    let key = format!("world:{}", world);
    let value = match db.get(key) {
        Ok(Some(value)) => value,
        _ => return Err(()),
    };

    match bincode::deserialize::<World>(&value) {
        Ok(response) => {
            if response.cached_at + chrono::Duration::minutes(5) < chrono::Utc::now() {
                return Err(());
            }
            Ok(response)
        }
        _ => Err(()),
    }
}

fn world_to_cache(db: sled::Db, world: i32, response: &World) {
    let key = format!("world:{}", world);
    let value = bincode::serialize(response).unwrap();
    db.insert(key, value).unwrap();
}
