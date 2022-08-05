# cdragon-rs

A Rust library for interacting with [CommunityDragon](https://www.communitydragon.org/).

Supports:
- Champion data
- Rune data

# Example Usage
```rust
use cdragon::repository::Repository;
use tokio;

#[tokio::main]
async fn main() {
    let repo = Repository::init().await.unwrap();

    let samira = repo.champions.get_by_id(360);
    match samira {
        Some(c) => {
            println!("Roles: {:?}", c.roles);
            println!(
                "Portrait URL: {}",
                c.square_portrait_path.map_path().unwrap()
            );
        }
        None => println!("None"),
    }
}
```

Output:
```text
Roles: ["marksman"]
Portrait URL: https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-icons/360.png
```
