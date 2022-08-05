use crate::asset::AssetURL;
use serde::Deserialize;
use std::collections::HashMap;

const RUNES_DATA_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perks.json";

/// Basic information about a Rune, as seen by CommunityDragon
#[derive(Deserialize, Debug)]
pub struct Rune {
    /// Internal rune id
    pub id: i32,
    /// Rune display name
    pub name: String,
    #[serde(rename = "iconPath")]
    /// Rune icon asset path
    pub icon_path: AssetURL,
}

/// Repository that contains [`Rune`] data
pub struct RunesRepository {
    /// HashMap of internal rune id values to [`Rune`] instances
    runes: HashMap<i32, Rune>,
}

impl RunesRepository {
    /// Loads the Rune data from CommunityDragon using the given HTTP client
    pub async fn load(client: &reqwest::Client) -> Result<RunesRepository, reqwest::Error> {
        let res = client
            .get(RUNES_DATA_URL)
            .header("Accept", "application/json")
            .send()
            .await?;

        let runes = match res.error_for_status() {
            Ok(res) => {
                let runes: Vec<Rune> = res.json().await?;
                runes
            }
            Err(err) => return Err(err),
        };

        let mut runes_map: HashMap<i32, Rune> = HashMap::new();
        for rune in runes {
            runes_map.insert(rune.id, rune);
        }

        Ok(RunesRepository { runes: runes_map })
    }

    /// Gets a [`Rune`] by its internal id
    pub fn get_by_id(&self, id: i32) -> Option<&Rune> {
        self.runes.get(&id)
    }
}
