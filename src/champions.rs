use serde::Deserialize;
use std::collections::HashMap;

const CHAMPION_DATA_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json";

/// Basic information about a Champion, as seen by CommunityDragon
#[derive(Deserialize, Debug)]
pub struct Champion {
    /// Internal champion id
    pub id: i32,
    /// Display champion name
    pub name: String,
    /// Internal alias for champion
    ///
    /// Sometimes differs from `name`, like in the case of `Wukong`
    pub alias: String,
    /// Champion portrait picture asset path
    #[serde(rename = "squarePortraitPath")]
    pub square_portrait_path: String,
    /// Vector of roles the champion is categorized under
    pub roles: Vec<String>,
}

/// Repository that contains [`Champion`] data
pub struct ChampionRepository {
    /// HashMap of internal champion id values to [`Champion`] instances
    champions: HashMap<i32, Champion>,
}

impl ChampionRepository {
    /// Loads the Champion data from CommunityDragon using the given HTTP client
    pub async fn load(client: &reqwest::Client) -> Result<ChampionRepository, reqwest::Error> {
        let res = client
            .get(CHAMPION_DATA_URL)
            .header("Accept", "application/json")
            .send()
            .await?;

        let champions = match res.error_for_status() {
            Ok(res) => {
                let champions: Vec<Champion> = res.json().await?;
                champions
            }
            Err(err) => return Err(err),
        };

        let mut champions_map: HashMap<i32, Champion> = HashMap::new();
        for champ in champions {
            champions_map.insert(champ.id, champ);
        }

        Ok(ChampionRepository {
            champions: champions_map,
        })
    }

    /// Gets a [`Champion`] by its internal id
    pub fn get_by_id(&self, id: i32) -> Option<&Champion> {
        self.champions.get(&id)
    }
}
