use bevy::{asset::Handle, asset::HandleId, asset::LoadState, prelude::*};

pub mod data_loaders;

pub struct ResourceLoaderPlugin;

impl Plugin for ResourceLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LoadingStatus::default())
            .add_system(asset_loading_system.system());
    }
}

#[derive(Default)]
pub struct LoadingStatus {
    pub items_loaded: usize,
    pub items_to_load: usize,
    pub initial_load_done: bool,
}

pub enum LoaderAssetType {
    Untyped,
    // TODO: UntypedDirectory,
    TextureWithId(u128),
}

/// A struct used to internally track texture loading progress.
/// Should not be added as a separate component.
pub struct LoadingProgressData {
    /// The path to load the asset from
    pub path: String,

    /// The type of asset to load. Currently only textures with an ID are separately defined
    pub asset_type: LoaderAssetType,

    /// the texture handle (used to track loading progress)
    handle: Option<HandleId>,

    /// Is set to true when the linked assets are fully loaded
    pub is_loaded: bool,
}

impl From<&str> for LoadingProgressData {
    fn from(path: &str) -> Self {
        LoadingProgressData {
            path: String::from(path),
            asset_type: LoaderAssetType::Untyped,
            handle: None,
            is_loaded: false,
        }
    }
}

impl From<(&str, u128)> for LoadingProgressData {
    fn from(data: (&str, u128)) -> Self {
        LoadingProgressData {
            path: String::from(data.0),
            asset_type: LoaderAssetType::TextureWithId(data.1),
            handle: None,
            is_loaded: false,
        }
    }
}

/// Stores paths to textures which will be loaded by the asset_loading_system
pub struct LoadAssets {
    pub assets: Vec<LoadingProgressData>,
}

fn asset_loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut loading_status: ResMut<LoadingStatus>,
    mut assets_to_load: Query<(Entity, &mut LoadAssets)>,
) {
    // trigger loading of new items
    for (entity, mut loader) in &mut assets_to_load.iter() {
        // drain filter, but not experimental
        let mut i = 0;
        while i < loader.assets.len() {
            let tex = &mut loader.assets[i];
            if tex.handle.is_none() {
                tex.handle = Some(asset_server.load_untyped(&tex.path).unwrap());
                loading_status.items_to_load += 1;
                i += 1;
                continue;
            }

            // check loading state
            let load_state = asset_server.get_load_state_untyped(tex.handle.unwrap());
            if load_state.is_none() {
                i += 1;
                continue;
            }

            // if loaded, check if we need to assign an ID
            tex.is_loaded = match load_state.unwrap() {
                LoadState::Loaded(_) => {
                    loading_status.items_loaded += 1;

                    match tex.asset_type {
                        LoaderAssetType::TextureWithId(id) => {
                            let asset = textures
                                .get(&Handle::from(tex.handle.unwrap()))
                                .unwrap()
                                .clone();
                            textures.set(Handle::from_u128(id), asset);
                        }
                        _ => {}
                    };

                    // texture is loaded, remove it
                    true
                }
                _ => false,
            };

            if tex.is_loaded {
                loader.assets.remove(i);
            } else {
                // texture is not loaded, check the next texture
                i += 1;
            }
        }

        if loader.assets.len() == 0 {
            println!("Despawning entity");
            commands.despawn(entity);
        }
    }

    // check if we are currently loading anything
    if loading_status.items_to_load == loading_status.items_loaded {
        if loading_status.initial_load_done == false {
            println!("Loading complete");
            loading_status.initial_load_done = true;
        }

        return;
    }

    println!(
        "Loaded {} of {} items",
        loading_status.items_loaded, loading_status.items_to_load
    );
}
