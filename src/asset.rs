use serde::{Deserialize, Serialize};

/// String wrapper for mapping asset urls
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct AssetURL(String);

const ASSET_PREFIX: &str = "/lol-game-data/assets/";
const TRANSFORMED_ASSET_PREFIX: &str =
    "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/";

impl AssetURL {
    /// Maps CommunityDragon asset paths to actual usable asset paths.
    ///
    /// If the given path is not detected as a valid asset path, `None` is returned.
    pub fn map_path(&self) -> Option<String> {
        if self.0.starts_with(ASSET_PREFIX) {
            let (_, path) = self.0.split_at(ASSET_PREFIX.len());
            Some(format!(
                "{}{}",
                TRANSFORMED_ASSET_PREFIX,
                path.to_lowercase()
            ))
        } else {
            None
        }
    }
}
