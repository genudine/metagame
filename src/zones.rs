use crate::{
    misc,
    types::{FactionPercents, Zone},
};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_aux::prelude::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref ZONE_REGIONS: HashMap<i32, Vec<i32>> = HashMap::from([
        (2, vec![2201, 2202, 2203]),
        (4, vec![4230, 4240, 4250]),
        (6, vec![6001, 6002, 6003]),
        (8, vec![18029, 18030, 18062]),
        (344, vec![18303, 18304, 18305]),
    ]);
}

pub async fn get_zone_states(world_id: i32) -> Result<Vec<Zone>, ()> {
    let response = reqwest::get(format!(
        "https://census.daybreakgames.com/s:{}/get/{}/map/?world_id={}&zone_ids=2,4,6,8,344",
        misc::service_id(),
        misc::platform(world_id),
        world_id,
    ))
    .await
    .unwrap();

    let map: MapResponse = match response.json().await {
        Ok(map) => map,
        Err(_) => return Err(()),
    };

    let mut zones: Vec<Zone> = Vec::new();

    for map_zone in map.map_list {
        let warpgate_zone_filter = ZONE_REGIONS.get(&map_zone.zone_id).unwrap();
        let warpgate_factions = map_zone
            .regions
            .row
            .iter()
            .filter(|r| warpgate_zone_filter.contains(&r.row_data.region_id))
            .map(|r| r.row_data.faction_id)
            .collect::<Vec<i32>>();

        let zone = Zone {
            id: map_zone.zone_id,
            locked: warpgate_factions[0] == warpgate_factions[1]
                && warpgate_factions[1] == warpgate_factions[2],
            territory: calculate_faction_percents(&map_zone.regions.row),
            alert: None,
        };

        zones.push(zone);
    }

    Ok(zones)
}

fn calculate_faction_percents(regions: &Vec<MapRegionRowData>) -> FactionPercents {
    let groups = regions.group_by(|a, b| a.row_data.faction_id == b.row_data.faction_id);

    struct FactionTotals {
        vs: f32,
        nc: f32,
        tr: f32,
    }

    let mut faction_totals = FactionTotals {
        vs: 0.0,
        nc: 0.0,
        tr: 0.0,
    };

    for row in groups {
        let faction_id = row[0].row_data.faction_id;

        match faction_id {
            1 => faction_totals.vs += 1.0,
            2 => faction_totals.nc += 1.0,
            3 => faction_totals.tr += 1.0,
            _ => (),
        }
    }

    let total = faction_totals.vs + faction_totals.nc + faction_totals.tr;

    FactionPercents {
        vs: faction_totals.vs / total * 100.0,
        nc: faction_totals.nc / total * 100.0,
        tr: faction_totals.tr / total * 100.0,
    }
}

#[derive(Deserialize)]
struct MapResponse {
    map_list: Vec<MapZone>,
}

#[derive(Deserialize)]
struct MapZone {
    #[serde(rename = "ZoneId", deserialize_with = "deserialize_number_from_string")]
    zone_id: i32,
    #[serde(rename = "Regions")]
    regions: MapRegionRow,
}

#[derive(Deserialize)]
struct MapRegionRow {
    #[serde(rename = "Row")]
    row: Vec<MapRegionRowData>,
}

#[derive(Deserialize)]
struct MapRegionRowData {
    #[serde(rename = "RowData")]
    row_data: MapRegion,
}

#[derive(Deserialize)]
struct MapRegion {
    #[serde(
        rename = "RegionId",
        deserialize_with = "deserialize_number_from_string"
    )]
    region_id: i32,

    #[serde(
        rename = "FactionId",
        deserialize_with = "deserialize_number_from_string"
    )]
    faction_id: i32,
}
