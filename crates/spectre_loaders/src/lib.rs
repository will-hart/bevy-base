use bevy::{asset::HandleId, asset::LoadState, prelude::*};
use std::iter;

pub struct ResourceLoaderPlugin;

impl Plugin for ResourceLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LoadingStatus {
            items_loaded: 0,
            items_to_load: 0,
            loading_handles: vec![],
        })
        .add_system(asset_loading_system.system());
    }
}

pub struct LoadingStatus {
    pub items_loaded: usize,
    pub items_to_load: usize,
    loading_handles: Vec<HandleId>,
}

/// Stores paths to assets which will be loaded by the asset_loading_system
pub struct AssetsToLoad {
    /// The path to load the asset from
    pub asset_paths: Vec<&'static str>,

    /// Optionally, a Vec of Optional u128 values to match IDs to the
    /// equivalent index in the asset_paths Vec. For empty or incomplete
    /// Vecs, None will be assumed for all remaining values
    pub asset_ids: Vec<Option<u128>>,
}

fn asset_loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_status: ResMut<LoadingStatus>,
    mut loading_query: Query<(Entity, &AssetsToLoad)>,
) {
    // trigger loading of new items
    for (entity, loader) in &mut loading_query.iter() {
        println!("Found a loader, adding {} assets", loader.asset_paths.len());

        let ids = loader
            .asset_ids
            .clone()
            .into_iter()
            .chain(iter::repeat(None));

        for (&path, id) in loader.asset_paths.iter().zip(ids) {
            println!("--> {}", path);
            let handle = asset_server.load_untyped(path).unwrap();

            if let Some(_) = id {
                println!("Should be applying an ID here!");
            }

            loading_status.items_to_load += 1;
            loading_status.loading_handles.push(handle);
        }

        println!("Despawning entity");
        commands.despawn(entity);
    }

    // check if we are currently loading anything
    if loading_status.items_to_load == loading_status.items_loaded {
        return;
    }

    let len_before = loading_status.loading_handles.len();

    // check loading progress
    loading_status.loading_handles = loading_status
        .loading_handles
        .as_slice()
        .iter()
        .filter(|&handle_id| {
            return match asset_server.get_load_state_untyped(*handle_id) {
                Some(load_state) => {
                    return match load_state {
                        LoadState::Loaded(_) => false,
                        _ => true,
                    }
                }
                None => true,
            };
        })
        .map(|h| *h)
        .collect::<Vec<_>>();

    loading_status.items_loaded += len_before - loading_status.loading_handles.len();
    println!(
        "{}/{} items loaded",
        loading_status.items_loaded, loading_status.items_to_load
    );
}
