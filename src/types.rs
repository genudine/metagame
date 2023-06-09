use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct World {
    pub id: i32,
    pub zones: Vec<Zone>,
    pub cached_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Zone {
    pub id: i32,
    pub locked: bool,
    pub alert: Option<Alert>,
    pub territory: FactionPercents,
    pub locked_since: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Alert {
    pub id: i32,
    pub zone: i32,
    pub end_time: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub alert_type: String,
    pub ps2alerts: String,
    pub percentages: FactionPercents,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct FactionPercents {
    pub vs: f32,
    pub nc: f32,
    pub tr: f32,
}
