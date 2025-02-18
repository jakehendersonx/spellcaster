use crate::config;
use crate::types::LoadPriority;
use macroquad::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// Represents the current storage location of a texture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureLocation {
    GPU,     // Texture is loaded in GPU memory (VRAM)
    RAM,     // Texture is in system RAM but not GPU
    Storage, // Texture is on disk/storage only
}

/// Metadata about a texture resource
pub struct TextureMetadata {
    location: TextureLocation,
    last_used: f64, // Timestamp of last usage
    loading: bool,  // Is this texture currently being loaded?
    size: usize,    // Size in bytes
}

/// Main texture manager that handles loading, caching, and memory management
pub struct TextureManager {
    // Active textures currently in GPU memory
    gpu_cache: HashMap<String, Arc<Texture2D>>,
    // Textures in system RAM (ready to be uploaded to GPU quickly)
    ram_cache: HashMap<String, Vec<u8>>,
    // Queue of texture IDs in order of rendering priority
    priority_queue: VecDeque<String>,
    // Metadata about all known textures
    metadata: HashMap<String, TextureMetadata>,
    // Procedurally generated textures for testing/placeholder
    procedural_textures: Vec<Arc<Texture2D>>,
    // Current memory usage
    gpu_memory_used: usize,
    ram_memory_used: usize,
}

impl TextureManager {
    /// Creates a new texture manager with empty caches
    pub async fn new() -> Self {
        let mut manager = TextureManager {
            gpu_cache: HashMap::with_capacity(config::MAX_GPU_TEXTURES),
            ram_cache: HashMap::with_capacity(config::MAX_RAM_TEXTURES),
            priority_queue: VecDeque::new(),
            metadata: HashMap::new(),
            procedural_textures: Vec::new(),
            gpu_memory_used: 0,
            ram_memory_used: 0,
        };

        // Generate our procedural textures
        for i in 0..4 {
            let texture = manager.generate_procedural_texture(i).await;
            // Estimate size (64x64 RGBA = 16384 bytes)
            manager.update_metadata(
                &format!("procedural_{}", i),
                TextureMetadata {
                    location: TextureLocation::GPU,
                    last_used: get_time(),
                    loading: false,
                    size: 16384,
                },
            );
            manager.procedural_textures.push(Arc::new(texture));
        }

        manager
    }

    /// Updates metadata for a texture
    fn update_metadata(&mut self, id: &str, metadata: TextureMetadata) {
        self.metadata.insert(id.to_string(), metadata);
    }

    /// Estimates the memory size of a texture
    fn estimate_texture_size(width: u32, height: u32, channels: u32) -> usize {
        (width * height * channels) as usize
    }

    /// Generates a procedural texture with unique patterns
    async fn generate_procedural_texture(&self, id: u32) -> Texture2D {
        let mut image =
            Image::gen_image_color(config::TILE_SIZE as u16, config::TILE_SIZE as u16, WHITE);

        match id % 4 {
            0 => {
                // Checkerboard pattern
                for y in 0..config::TILE_SIZE as u16 {
                    for x in 0..config::TILE_SIZE as u16 {
                        if (x / 8 + y / 8) % 2 == 0 {
                            image.set_pixel(x, y, BLUE);
                        }
                    }
                }
            }
            1 => {
                // Circle pattern
                let center = config::TILE_SIZE as f32 / 2.0;
                let radius = config::TILE_SIZE as f32 / 3.0;
                for y in 0..config::TILE_SIZE as u16 {
                    for x in 0..config::TILE_SIZE as u16 {
                        let dx = x as f32 - center;
                        let dy = y as f32 - center;
                        if (dx * dx + dy * dy).sqrt() < radius {
                            image.set_pixel(x, y, GREEN);
                        }
                    }
                }
            }
            2 => {
                // Diagonal stripes pattern
                for y in 0..config::TILE_SIZE as u16 {
                    for x in 0..config::TILE_SIZE as u16 {
                        if (x as i32 + y as i32) % 16 < 8 {
                            image.set_pixel(x, y, RED);
                        }
                    }
                }
            }
            3 => {
                // Dot pattern
                for y in 0..config::TILE_SIZE as u16 {
                    for x in 0..config::TILE_SIZE as u16 {
                        if x % 8 == 0 && y % 8 == 0 {
                            for dy in 0..4 {
                                for dx in 0..4 {
                                    if x + dx < config::TILE_SIZE as u16
                                        && y + dy < config::TILE_SIZE as u16
                                    {
                                        image.set_pixel(x + dx, y + dy, YELLOW);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        Texture2D::from_image(&image)
    }

    /// Gets a texture for a specific world position
    pub fn get_texture(&self, x: i32, y: i32) -> Arc<Texture2D> {
        // For testing, we'll use procedural textures based on position
        let index = ((x.abs() + y.abs()) % 4) as usize;
        Arc::clone(&self.procedural_textures[index])
    }

    /// Ensures a texture is loaded with the appropriate priority
    pub async fn ensure_loaded(
        &mut self,
        id: &str,
        priority: LoadPriority,
    ) -> Option<Arc<Texture2D>> {
        // Update last used time if we have metadata
        if let Some(metadata) = self.metadata.get_mut(id) {
            metadata.last_used = get_time();
        }

        // Check GPU cache first
        if let Some(texture) = self.gpu_cache.get(id).cloned() {
            self.update_priority(id, priority == LoadPriority::Immediate);
            return Some(texture);
        }

        // If in RAM and high priority, move to GPU
        if (priority == LoadPriority::Immediate || priority == LoadPriority::Preload)
            && self.ram_cache.contains_key(id)
        {
            // Get the data before we start modifying anything
            let data = self.ram_cache.get(id).unwrap().clone();

            // Ensure we have room in GPU memory
            if self.gpu_cache.len() >= config::MAX_GPU_TEXTURES {
                self.evict_least_important();
            }

            // Create the new texture
            let texture = Arc::new(Texture2D::from_file_with_format(&data, None));

            // Update metadata
            if let Some(metadata) = self.metadata.get_mut(id) {
                metadata.location = TextureLocation::GPU;
                metadata.loading = false;
            }

            // Store in GPU cache and update priority
            self.gpu_cache.insert(id.to_string(), Arc::clone(&texture));
            self.update_priority(id, priority == LoadPriority::Immediate);

            return Some(texture);
        }

        // If not in RAM and high priority, load from storage
        if priority == LoadPriority::Immediate {
            // Update loading state in metadata
            if let Some(metadata) = self.metadata.get_mut(id) {
                metadata.loading = true;
            }

            // Load the texture
            match self.load_from_storage(id).await {
                Some(new_texture) => {
                    let texture = Arc::new(new_texture);

                    // Update metadata after loading
                    if let Some(metadata) = self.metadata.get_mut(id) {
                        metadata.location = TextureLocation::GPU;
                        metadata.loading = false;
                    }

                    // Store in cache and update priority
                    self.gpu_cache.insert(id.to_string(), Arc::clone(&texture));
                    self.update_priority(id, true);

                    Some(texture)
                }
                None => None,
            }
        } else {
            None
        }
    }
    /// Updates the priority of a texture in the queue
    fn update_priority(&mut self, id: &str, high_priority: bool) {
        self.priority_queue.retain(|queued_id| queued_id != id);
        if high_priority {
            self.priority_queue.push_front(id.to_string());
        } else {
            self.priority_queue.push_back(id.to_string());
        }
    }

    /// Evicts the least important texture from GPU to RAM
    fn evict_least_important(&mut self) {
        while let Some(id) = self.priority_queue.pop_back() {
            if let Some(texture) = self.gpu_cache.remove(&id) {
                // Get texture data before potentially dropping the last reference
                let data = Arc::as_ref(&texture).get_texture_data();

                // Update metadata
                if let Some(metadata) = self.metadata.get_mut(&id) {
                    metadata.location = TextureLocation::RAM;
                    self.ram_memory_used += metadata.size;
                    self.gpu_memory_used -= metadata.size;
                }

                // Ensure RAM cache has room
                while self.ram_cache.len() >= config::MAX_RAM_TEXTURES {
                    if let Some(oldest_id) = self.priority_queue.pop_back() {
                        if let Some(metadata) = self.metadata.get(&oldest_id) {
                            self.ram_memory_used -= metadata.size;
                        }
                        self.ram_cache.remove(&oldest_id);
                    }
                }

                self.ram_cache.insert(id, data);
                break;
            }
        }
    }

    /// Loads a texture from storage (simulated for now)
    async fn load_from_storage(&mut self, _id: &str) -> Option<Texture2D> {
        // Simulate loading delay
        next_frame().await;

        // For now, return a dummy texture
        // In a real implementation, this would load from disk
        Some(self.generate_procedural_texture(0).await)
    }

    /// Gets statistics about current memory usage
    pub fn get_memory_stats(&self) -> (usize, usize) {
        (self.gpu_memory_used, self.ram_memory_used)
    }

    /// Cleans up unused textures
    pub fn cleanup_unused(&mut self) {
        let current_time = get_time();
        let timeout = 30.0; // 30 seconds unused = cleanup

        self.gpu_cache.retain(|id, _| {
            if let Some(metadata) = self.metadata.get(id) {
                current_time - metadata.last_used < timeout
            } else {
                false
            }
        });

        self.ram_cache.retain(|id, _| {
            if let Some(metadata) = self.metadata.get(id) {
                current_time - metadata.last_used < timeout
            } else {
                false
            }
        });
    }
}
