use std::{
    any::{self, Any, TypeId},
    collections::HashMap,
    error::Error,
    fs,
    sync::Arc,
};

use crate::gl::GraphicsContext;

pub mod texture;

pub struct Assets {
    caches: HashMap<TypeId, Box<dyn Any>>,
}
struct AssetCache<L> {
    loaded: HashMap<&'static str, Arc<L>>,
}

impl Assets {
    pub(crate) fn new() -> Self {
        Assets {
            caches: HashMap::new(),
        }
    }

    pub fn load<L>(&mut self, path: &'static str) -> Arc<L>
    where
        L: Asset,
    {
        let key = TypeId::of::<L>();
        let cache = self.caches.get_mut(&key);
        let cache: &mut AssetCache<L> = if let Some(cache) = cache {
            cache
        } else {
            self.caches.insert(
                key,
                Box::new(AssetCache::<L> {
                    loaded: HashMap::new(),
                }),
            );
            self.caches.get_mut(&key).unwrap()
        }
        .downcast_mut()
        .expect("valid cache in asset loader");

        if let Some(asset) = cache.loaded.get(&path) {
            asset.clone()
        } else {
            let data = fs::read(path);
            if let Ok(data) = data {
                let arc = Arc::new(if let Ok(asset) = L::load(&data) {
                    asset
                } else {
                    panic!(
                        "Failed to parse {asset} from path {path}.",
                        asset = any::type_name::<L>(),
                    )
                });
                cache.loaded.insert(path, arc.clone());
                arc
            } else {
                panic!("Could not read file {path}.")
            }
        }
    }
}

pub trait Asset: 'static + Sized {
    fn load(data: &[u8]) -> Result<Self, Box<dyn Error>>;
}

impl Assets {
    pub(crate) fn load_gl<L>(&mut self, path: &'static str, gl: &mut GraphicsContext) -> Arc<L>
    where
        L: GlAsset,
    {
        let key = TypeId::of::<L>();
        let cache = self.caches.get_mut(&key);
        let cache: &mut AssetCache<L> = if let Some(cache) = cache {
            cache
        } else {
            self.caches.insert(
                key,
                Box::new(AssetCache::<L> {
                    loaded: HashMap::new(),
                }),
            );
            self.caches.get_mut(&key).unwrap()
        }
        .downcast_mut()
        .expect("valid cache in asset loader");

        if let Some(asset) = cache.loaded.get(&path) {
            asset.clone()
        } else {
            let data = fs::read(path);
            if let Ok(data) = data {
                let arc = Arc::new(if let Ok(asset) = L::load(&data, gl) {
                    asset
                } else {
                    panic!(
                        "Failed to parse {asset} from path {path}.",
                        asset = any::type_name::<L>(),
                    )
                });
                cache.loaded.insert(path, arc.clone());
                arc
            } else {
                panic!("Could not read file {path}.")
            }
        }
    }
}

pub(crate) trait GlAsset: 'static + Sized {
    fn load(data: &[u8], gl: &mut GraphicsContext) -> Result<Self, Box<dyn Error>>;
}
