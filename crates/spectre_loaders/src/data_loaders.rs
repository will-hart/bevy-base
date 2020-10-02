use bevy::asset::AssetLoader;
use ron::de::from_bytes;
use serde::de::Deserialize;
use std::path::Path;

/// A generic data file loader which loads RON files from the assets folder
/// and deserializes them into the provided type. There should be a 1:1 mapping
/// between the file extension set and the type of asset loaded
#[derive(Default)]
pub struct DataFileLoader {
    matching_extensions: Vec<&'static str>,
}

impl DataFileLoader {
    pub fn from_extensions(matching_extensions: Vec<&'static str>) -> Self {
        DataFileLoader {
            matching_extensions,
        }
    }
}

impl<TAsset> AssetLoader<TAsset> for DataFileLoader
where
    for<'de> TAsset: Deserialize<'de>,
{
    fn from_bytes(&self, _asset_path: &Path, bytes: Vec<u8>) -> Result<TAsset, anyhow::Error> {
        Ok(from_bytes::<TAsset>(bytes.as_slice())?)
    }

    fn extensions(&self) -> &[&str] {
        self.matching_extensions.as_slice()
    }
}
