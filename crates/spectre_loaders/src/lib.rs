use bevy::{asset::Handle, asset::LoadState, prelude::*};

pub struct ResourceLoaderPlugin;

impl Plugin for ResourceLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LoadingStatus::default())
            .add_system(texture_loading_system.system());
    }
}

#[derive(Default)]
pub struct LoadingStatus {
    pub items_loaded: usize,
    pub items_to_load: usize,
    pub initial_load_done: bool,
}

pub struct TextureLoadingData {
    /// The path to load the asset from
    pub path: String,

    /// an optional ID to assign to the texture once loaded
    pub id: Option<u128>,

    /// the texture handle (used to track loading progress)
    pub handle: Option<Handle<Texture>>,

    pub is_loaded: bool,
}

impl From<&str> for TextureLoadingData {
    fn from(path: &str) -> Self {
        TextureLoadingData {
            path: String::from(path),
            id: None,
            handle: None,
            is_loaded: false,
        }
    }
}

impl From<(&str, u128)> for TextureLoadingData {
    fn from(data: (&str, u128)) -> Self {
        TextureLoadingData {
            path: String::from(data.0),
            id: Some(data.1),
            handle: None,
            is_loaded: false,
        }
    }
}

/// Stores paths to textures which will be loaded by the asset_loading_system
pub struct TexturesToLoad {
    pub textures: Vec<TextureLoadingData>,
}

fn texture_loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut loading_status: ResMut<LoadingStatus>,
    mut textures_to_load: Query<(Entity, &mut TexturesToLoad)>,
) {
    // trigger loading of new items
    for (entity, mut loader) in &mut textures_to_load.iter() {
        // drain filter, but not experimental
        let mut i = 0;
        while i < loader.textures.len() {
            let tex = &mut loader.textures[i];
            let handle = tex.handle;
            if handle.is_none() {
                tex.handle = Some(asset_server.load(tex.path.clone()).unwrap());
                loading_status.items_to_load += 1;
                i += 1;
                continue;
            }

            // check loading state
            let load_state = asset_server.get_load_state(handle.unwrap());
            if load_state.is_none() {
                i += 1;
                continue;
            }

            // if loaded, check if we need to assign an ID
            tex.is_loaded = match load_state.unwrap() {
                LoadState::Loaded(_) => {
                    loading_status.items_loaded += 1;

                    if tex.id.is_some() {
                        // can only match up the ID here
                        let asset = textures.get(&handle.unwrap()).unwrap().clone();
                        textures.set(Handle::from_u128(tex.id.unwrap()), asset);
                    }

                    // texture is loaded, remove it
                    true
                }
                _ => false,
            };

            if tex.is_loaded {
                loader.textures.remove(i);
            } else {
                // texture is not loaded, check the next texture
                i += 1;
            }
        }

        if loader.textures.len() == 0 {
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
        "Loaded {} of {} textures",
        loading_status.items_loaded, loading_status.items_to_load
    );
}
