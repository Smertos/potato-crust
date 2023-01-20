use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use thiserror::Error;
use crate::block_texture::BlockTexture;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("asset with name `{0}` already exists in the registry")]
    AssetExists(String),
}

pub struct Registry<T: Asset> {
    data: HashMap<String, Handle<T>>,
}

impl<T: Asset> Registry<T> {
    pub fn new(initial_capacity: Option<usize>) -> Self {
        Self {
            data: HashMap::with_capacity(initial_capacity.unwrap_or(0)),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, asset_name: &str) -> Option<Handle<T>> {
        self.data.get(asset_name).map(Handle::clone_weak)
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, asset_name: impl Into<String>, asset: Handle<T>) -> Result<(), RegistryError> {
        let asset_name: String = asset_name.into();

        if !self.data.contains_key(&asset_name) {
            self.data.insert(asset_name, asset);

            Ok(())
        } else {
            Err(RegistryError::AssetExists(asset_name))
        }
    }
}

macro_rules! make_registry {
    ($name:ident, $asset:ident, $uuid:literal) => {
        #[derive(Resource, TypeUuid)]
        #[uuid = $uuid]
        pub struct $name(pub Registry<$asset>);

        impl $name {
            #[allow(dead_code)]
            pub fn get(&self, asset_name: &str) -> Option<Handle<$asset>> {
                self.0.get(asset_name)
            }

            #[allow(dead_code)]
            pub fn insert(&mut self, asset_name: impl Into<String>, asset: Handle<$asset>) -> Result<(), RegistryError> {
                self.0.insert(asset_name, asset)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name(Registry::<$asset>::new(None))
            }
        }
    }
}

// TODO: automate struct & Default impl below with a macro
make_registry!(BlockTextureRegistry, BlockTexture, "19d77a98-2675-4306-ab3c-ddc39ada1ffd");

// #[derive(Resource, TypeUuid)]
// #[uuid = "19d77a98-2675-4306-ab3c-ddc39ada1ffd"]
// pub struct BlockTextureRegistry(pub Registry<BlockTexture>);
//
// impl<T: Asset> Default for BlockTextureRegistry {
//     fn default() -> Self {
//         BlockTextureRegistry(Registry::<T>::new(None))
//     }
// }
