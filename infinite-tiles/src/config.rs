/// config.rs
/// configuration for chunk rendering
/// gpu textures
/// player speed
/// cache loading

pub const TILE_SIZE: i32 = 64;
pub const CHUNK_SIZE: i32 = 16;
pub const VISIBLE_CHUNKS_RADIUS: i32 = 2;
pub const PRELOAD_CHUNKS_RADIUS: i32 = 3;
pub const CACHE_CHUNKS_RADIUS: i32 = 4;
pub const PLAYER_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 5.0;
pub const MAX_GPU_TEXTURES: usize = 64;
pub const MAX_RAM_TEXTURES: usize = 256;
