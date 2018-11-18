use ::prelude::*;

gw2rs_id_u64!(ColorId);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Color {
    pub id: ColorId,
    pub name: String,
    pub base_rgb: [u64; 3],
    pub cloth: ColorDetails,
    pub leather: ColorDetails,
    pub metal: ColorDetails,
    pub item: ItemId,
    pub categories: Vec<String>, // TODO: turn this into a set of enum variants or something instead
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ColorDetails {
    pub brightness: i64,
    pub contrast: f64,
    pub hue: i64,
    pub saturation: f64,
    pub lightness: f64,
    pub rgb: [u64; 3],
}
