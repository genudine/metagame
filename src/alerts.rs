use crate::{
    alert_types::alert_type,
    misc,
    types::{Alert, FactionPercents},
};
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;

pub async fn get_alerts(world_id: i32) -> Result<(Vec<Alert>, Vec<Alert>), ()> {
    let response = reqwest::get(format!(
        "https://census.daybreakgames.com/s:{}/get/{}/world_event/?world_id={}&type=METAGAME&c:limit=15",
        misc::service_id(),
        misc::platform(world_id),
        world_id
    ))
    .await
    .unwrap();

    let world_events: WorldEventResponse = match response.json().await {
        Ok(world_events) => world_events,
        Err(_) => return Err(()),
    };

    let mut alerts: HashMap<i32, Alert> = HashMap::new();

    for world_event in world_events.world_event_list {
        let alert = alerts.entry(world_event.id).or_insert(Alert {
            id: world_event.id,
            zone: world_event.zone_id,
            end_time: None,
            start_time: None,
            alert_type: alert_type(world_event.metagame_event_id),
            ps2alerts: format!(
                "https://ps2alerts.com/alert/{}-{}",
                world_id, world_event.id
            ),
            percentages: FactionPercents {
                nc: world_event.faction_nc,
                tr: world_event.faction_tr,
                vs: world_event.faction_vs,
            },
        });

        if world_event.metagame_event_state_name == "started" {
            alert.start_time = Utc.timestamp_opt(world_event.timestamp as i64, 0).single();
        } else if world_event.metagame_event_state_name == "ended" {
            alert.end_time = Utc.timestamp_opt(world_event.timestamp as i64, 0).single();
        }
    }

    let alerts = alerts
        .into_iter()
        .map(|(_, alert)| alert)
        .collect::<Vec<Alert>>();

    let newest_alert_by_zone = alerts
        .clone()
        .into_iter()
        .fold(HashMap::<i32, Alert>::new(), |mut map, alert| {
            if let Some(current_alert) = map.get_mut(&alert.zone) {
                if alert.start_time > current_alert.start_time {
                    *current_alert = alert;
                }
            } else {
                map.insert(alert.zone, alert);
            }

            map
        })
        .into_iter()
        .map(|(_, alert)| alert)
        .collect::<Vec<Alert>>();

    let active_alerts: Vec<Alert> = alerts
        .clone()
        .into_iter()
        .filter(|alert| alert.end_time.is_none())
        .collect();

    Ok((active_alerts, newest_alert_by_zone))
}

#[derive(Deserialize)]
struct WorldEventResponse {
    world_event_list: Vec<WorldEvent>,
}

#[derive(Deserialize)]
struct WorldEvent {
    #[serde(
        rename = "instance_id",
        deserialize_with = "deserialize_number_from_string"
    )]
    id: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    metagame_event_id: i32,
    metagame_event_state_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    timestamp: i64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    zone_id: i32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    faction_nc: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    faction_tr: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    faction_vs: f32,
}
