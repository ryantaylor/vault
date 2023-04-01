use data::chunks::DataSdscChunk;

#[derive(Debug)]
pub struct Map {
    filename: String,
    localized_name_id: String,
    localized_description_id: String,
}

impl Map {
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn localized_name_id(&self) -> &str {
        &self.localized_name_id
    }
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
