use serde::Deserialize;
use std::collections::HashMap;

const RUNES_DATA_URL: &str = "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/perks.json";

#[derive(Deserialize, Debug)]
pub struct Rune {
    pub id: i32,
    pub name: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

pub struct RunesRepository {
    runes: HashMap<i32, Rune>,
}

impl RunesRepository {
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

    pub fn get_by_id(&self, id: i32) -> Option<&Rune> {
        self.runes.get(&id)
    }
}
