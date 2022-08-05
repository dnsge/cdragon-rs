use crate::champions::ChampionRepository;
use crate::runes::RunesRepository;

use reqwest::header::{HeaderMap, HeaderValue};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn user_agent() -> String {
    format!("cdragon-rs/{}", VERSION)
}

fn default_client() -> reqwest::Client {
    let mut default_headers = HeaderMap::new();
    let ua = HeaderValue::try_from(user_agent()).unwrap();
    default_headers.insert("User-Agent", ua);
    reqwest::Client::builder()
        .default_headers(default_headers)
        .build()
        .unwrap()
}

pub struct Repository {
    client: reqwest::Client,
    pub champions: ChampionRepository,
    pub runes: RunesRepository,
}

impl Repository {
    pub async fn init() -> Result<Repository, reqwest::Error> {
        Repository::init_with_client(default_client()).await
    }

    pub async fn init_with_client(client: reqwest::Client) -> Result<Repository, reqwest::Error> {
        let champions = ChampionRepository::load(&client).await?;
        let runes = RunesRepository::load(&client).await?;
        Ok(Repository {
            client,
            champions,
            runes,
        })
    }
}

const ASSET_PREFIX: &str = "/lol-game-data/assets/";
const TRANSFORMED_ASSET_PREFIX: &str =
    "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/";

pub fn map_asset_path(path: &str) -> Option<String> {
    if path.starts_with(ASSET_PREFIX) {
        let (_, path) = path.split_at(ASSET_PREFIX.len());
        Some(format!(
            "{}{}",
            TRANSFORMED_ASSET_PREFIX,
            path.to_lowercase()
        ))
    } else {
        None
    }
}
