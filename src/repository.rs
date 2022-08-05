use crate::champions::ChampionRepository;
use crate::runes::RunesRepository;

use reqwest::header::{HeaderMap, HeaderValue};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns a `User-Agent` header string with the package version
fn user_agent() -> String {
    format!("cdragon-rs/{}", VERSION)
}

/// Returns a [`reqwest::Client`] configured with a default `User-Agent` header
fn default_client() -> reqwest::Client {
    let mut default_headers = HeaderMap::new();
    let ua = HeaderValue::try_from(user_agent()).unwrap();
    default_headers.insert("User-Agent", ua);
    reqwest::Client::builder()
        .default_headers(default_headers)
        .build()
        .unwrap()
}

/// A `Repository` stores CommunityDragon data for multiple providers.
///
/// Currently, [`ChampionRepository`] provides `Champion` data and
/// [`RunesRepository`] provides `Rune` data.
///
/// You can initialize a new `Repository` with [`Repository::init`], which
/// uses a default HTTP client, or you can use your own client with
/// [`Repository::init_with_client`].
///
/// # Example
/// ```
/// use cdragon::repository::Repository;
///
/// // load data from CommunityDragon
/// let repo = Repository::init().await.unwrap();
/// // find champion by id
/// let samira = repo.champions.get_by_id(360);
/// ```
pub struct Repository {
    client: reqwest::Client,
    pub champions: ChampionRepository,
    pub runes: RunesRepository,
}

impl Repository {
    /// Initializes the `Repository` using a default HTTP client
    pub async fn init() -> Result<Repository, reqwest::Error> {
        Repository::init_with_client(default_client()).await
    }

    /// Initializes the `Repository` using the given HTTP client
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

/// Maps CommunityDragon asset paths to actual usable asset paths.
///
/// If the given path is not detected as a valid asset path, `None` is returned.
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
