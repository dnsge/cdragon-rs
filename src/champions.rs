use serde::Deserialize;
use std::collections::HashMap;

const CHAMPION_DATA_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json";

#[derive(Deserialize, Debug)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub alias: String,
    #[serde(rename = "squarePortraitPath")]
    pub square_portrait_path: String,
    pub roles: Vec<String>,
}

pub struct ChampionRepository {
    champions: HashMap<i32, Champion>,
}

impl ChampionRepository {
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

    pub fn get_by_id(&self, id: i32) -> Option<&Champion> {
        self.champions.get(&id)
    }
}
