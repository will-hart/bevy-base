use bevy::prelude::*;
use serde::Deserialize;
use spectre_loaders::data_loaders::DataFileLoader;

#[derive(Deserialize)]
pub struct Character {
    pub test: i32,
}

pub struct DataFileLoaderPlugin;

impl Plugin for DataFileLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Character>()
            .add_asset_loader_from_instance::<Character, DataFileLoader>(
                DataFileLoader::from_extensions(vec!["chd"]),
            );
    }
}
