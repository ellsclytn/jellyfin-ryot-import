#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SeenHistory {
    pub show_episode_number: Option<u32>,
    pub show_season_number: Option<u32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Item {
    pub collections: Vec<String>,
    pub identifier: String,
    pub lot: String,
    pub reviews: Vec<String>,
    pub seen_history: Vec<SeenHistory>,
    pub source: String,
    pub source_id: String,
}
