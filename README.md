# screeps-rust-api

[English](README.md) | [中文](README.zh.md)

A Rust API client library for the Screeps game.

Currently in rapid development. **Many features are still incomplete, stay tuned!**

## Features

- Async HTTP client support
- Automatic rate limiting
- Screeps API wrappers
- Authentication and Token management

## Usage Example

```rust
use screeps_rust_api::{ScreepsApi, ScreepsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ScreepsConfig::new(
        Some("your_token".to_string()),
        None,
        None,
        "https://screeps.com".to_string(),
        true,
        10,
    );

    let api = ScreepsApi::new(config);

    // Get user info
    let user_info = api.get_my_info().await?;
    println!("User ID: {}", user_info.user.unwrap()._id);

    // Get room objects
    let room_objects = api.get_room_objects("E13S13", "shard3").await?;
    println!(
        "Found {} objects in room",
        room_objects.objects.unwrap().len()
    );

    Ok(())
}
```

For more, see the examples under the `examples` directory.

## Supported API Endpoints

### User

- `get_my_info()` - Get current user info
- `get_my_name()` - Get current username
- `get_user_info_by_name(username)` - Get user info by username
- `get_user_info_by_id(id)` - Get user info by user ID
- `get_user_rooms(id)` - Get all rooms for a given user

### Room

- `get_room_objects(room, shard)` - Get all objects in a room
- `get_room_terrain(room, shard)` - Get room terrain info
- `get_room_terrain_encoded(room, shard)` - Get encoded room terrain info
- `get_room_status(room, shard)` - Get room status

### Game

- `get_shards()` - Get all shard info
- `get_shard_time(shard)` - Get the current game time for a shard

### Authentication

- `auth()` - Authenticate and get token

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

Note: Some tests require valid Screeps account credentials, which are provided via environment variables.

To run tests requiring authentication, create a `.env` file with the following variables:

```env
SCREEPS_EMAIL=your_email@example.com
SCREEPS_PASSWORD=your_password
SCREEPS_TOKEN=your_token
```
