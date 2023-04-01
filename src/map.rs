//! Representation of parsed map information.

use data::chunks::DataSdscChunk;

/// Representation of all map-related information that can be parsed from a Company of Heroes 3
/// replay

#[derive(Debug, Clone)]
#[cfg_attr(feature = "magnus", magnus::wrap(class = "Vault::Map"))]
pub struct Map {
    filename: String,
    localized_name_id: String,
    localized_description_id: String,
}

impl Map {
    /// This is a "filename" in the sense that its structure resembles one, but it doesn't actually
    /// point to any file on the file system. The final "token" in this string (if you split by
    /// slash) generally corresponds to the map name returned by the CoH3 stats API. The string is
    /// UTF-8 encoded.
    pub fn filename(&self) -> &str { &self.filename }
    /// Entity ID that corresponds to a localization string that represents the localized name of
    /// the map. Conventionally these IDs do not change between patches, but that isn't guaranteed.
    /// The string is UTF-16 encoded.
    pub fn localized_name_id(&self) -> &str {
        &self.localized_name_id
    }
    /// Entity ID that corresponds to a localization string that represents the localized
    /// description of the map. Conventionally these IDs do not change between patches, but that
    /// isn't guaranteed. The string is UTF-16 encoded.
    pub fn localized_description_id(&self) -> &str {
        &self.localized_description_id
    }
}

pub fn map_from_data(data: &DataSdscChunk) -> Map {
    Map {
        filename: data.map_file.clone(),
        localized_name_id: data.map_name.clone(),
        localized_description_id: data.map_description.clone(),
    }
}
