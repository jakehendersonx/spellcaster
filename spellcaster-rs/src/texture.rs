use crate::config;
use crate::types::LoadPriority;
use macroquad::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

pub struct TextureManager {
    gpu_cache: HashMap<String, Arc<Texture2D>>,
    ram_cache: HashMap<String, Vec<u8>>,
    priority_queue: VecDeque<String>,
    procedural_textures: Vec<Arc<Texture2D>>,
}

impl TextureManager {
    pub async fn new() -> Self {
        let mut manager = TextureManager {
            gpu_cache: HashMap::with_capacity(config::MAX_GPU_TEXTURES),
            ram_cache: HashMap::with_capacity(config::MAX_RAM_TEXTURES),
            priority_queue: VecDeque::new(),
            procedural_textures: Vec::new(),
        };

        // Generate our procedural textures
        for i in 0..4 {
            let texture = manager.generate_procedural_texture(i).await;
            manager.procedural_textures.push(Arc::new(texture));
        }

        manager
    }

    pub fn get_texture(&self, x: i32, y: i32) -> Arc<Texture2D> {
        let index = ((x.abs() + y.abs()) % 4) as usize;
        Arc::clone(&self.procedural_textures[index])
    }

    pub async fn ensure_loaded(
        &mut self,
        id: &str,
        priority: LoadPriority,
    ) -> Option<Arc<Texture2D>> {
        // If in GPU, update priority and return clone of Arc
        if let Some(texture) = self.gpu_cache.get(id) {
            self.update_priority(id, priority == LoadPriority::Immediate);
            return Some(Arc::clone(texture));
        }

        // If in RAM and high priority, move to GPU
        if priority == LoadPriority::Immediate || priority == LoadPriority::Preload {
            if let Some(data) = self.ram_cache.get(id) {
                let data = data.clone();
                if self.gpu_cache.len() >= config::MAX_GPU_TEXTURES {
                    self.evict_least_important();
                }

                let texture = Arc::new(Texture2D::from_file_with_format(&data, None));
                self.gpu_cache.insert(id.to_string(), Arc::clone(&texture));
                self.update_priority(id, priority == LoadPriority::Immediate);
                return Some(texture);
            }
        }

        // If not in RAM and high priority, load from storage
        if priority == LoadPriority::Immediate {
            let texture = Arc::new(self.load_from_storage(id).await?);
            self.gpu_cache.insert(id.to_string(), Arc::clone(&texture));
            self.update_priority(id, true);
            Some(texture)
        } else {
            None
        }
    }

    fn evict_least_important(&mut self) {
        if let Some(id) = self.priority_queue.pop_back() {
            // Arc will automatically handle cleanup when the last reference is dropped
            if let Some(texture) = self.gpu_cache.remove(&id) {
                // We can get the texture data because Arc gives us shared access
                let data = Arc::as_ref(&texture).get_texture_data();
                if self.ram_cache.len() >= config::MAX_RAM_TEXTURES {
                    if let Some(oldest_id) = self.priority_queue.pop_back() {
                        self.ram_cache.remove(&oldest_id);
                    }
                }
                self.ram_cache.insert(id, data);
            }
        }
    }

    // ... rest of the implementation remains the same ...
}
