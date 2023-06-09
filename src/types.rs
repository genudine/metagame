use serde::Serialize;

#[derive(Serialize)]
pub struct World {
    pub id: i32,
    pub zones: Vec<Zone>,
}

#[derive(Serialize)]
pub struct Zone {
    pub id: i32,
    pub locked: bool,
    pub alert: Option<Alert>,
    pub faction_control: FactionPercents,
}

#[derive(Serialize)]
pub struct Alert {
    pub id: i32,
    #[serde(rename = "type")]
    pub alert_type: String,
    pub start: i64,
    pub end: i64,
    pub score: FactionPercents,
    pub ps2alerts: String,
}

#[derive(Serialize)]
pub struct FactionPercents {
    pub vs: f32,
    pub nc: f32,
    pub tr: f32,
}
