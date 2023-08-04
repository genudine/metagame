use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize)]
struct MetagameEventResponse {
    metagame_event_list: Vec<MetagameEvent>,
}

#[derive(Deserialize)]
struct MetagameEvent {
    metagame_event_id: String,
    #[serde(rename = "type", deserialize_with = "deserialize_number_from_string")]
    event_type: i32,
    name: LangEn,
}

#[derive(Deserialize)]
struct LangEn {
    en: String,
}

#[tokio::main]
async fn main() {
    let response = reqwest::get(
        "https://census.daybreakgames.com/s:ps2livepublic/get/ps2/metagame_event?c:limit=1000",
    )
    .await
    .unwrap();

    let metagame_events: MetagameEventResponse = response.json().await.unwrap();

    let template = format!("// GENERATED CODE. DO NOT EDIT MANUALLY. Run `cd hack/metagame-gen; cargo run` to generate.

pub fn alert_type(metagame_event_id: i32) -> String {{
    match metagame_event_id {{
        {} => \"air\".to_string(),
        {} => \"max\".to_string(),
        {} => \"sudden_death\".to_string(),
        {} => \"unstable_meltdown\".to_string(),
        {} | _ => \"conquest\".to_string(),
    }}
}}",
        metagame_events.metagame_event_list.iter().filter(|e| e.event_type == 10).map(|e| e.metagame_event_id.clone()).collect::<Vec<String>>().join(" | "),
        metagame_events.metagame_event_list.iter().filter(|e| e.event_type == 6 && e.name.en.contains("aximum")).map(|e| e.metagame_event_id.clone()).collect::<Vec<String>>().join(" | "),
        metagame_events.metagame_event_list.iter().filter(|e| e.event_type == 6 && !e.name.en.contains("aximum")).map(|e| e.metagame_event_id.clone()).collect::<Vec<String>>().join(" | "),
        metagame_events.metagame_event_list.iter().filter(|e| e.event_type == 9 && e.name.en.contains("eltdown")).map(|e| e.metagame_event_id.clone()).collect::<Vec<String>>().join(" | "),
        metagame_events.metagame_event_list.iter().filter(|e| e.event_type == 9 && !e.name.en.contains("eltdown")).map(|e| e.metagame_event_id.clone()).collect::<Vec<String>>().join(" | "),
    );

    std::fs::write("../../src/alert_types.rs", template).unwrap();

    println!("Generated alert_types.rs");
}
