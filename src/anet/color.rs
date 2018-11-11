use ::prelude::*;

gw2rs_id_u64!(ColorId);

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Color {
    id: ColorId,
    name: String,
    base_rgb: [u64; 3],
    cloth: ColorDetails,
    leather: ColorDetails,
    metal: ColorDetails,
    item: ItemId,
    categories: Vec<String>, // TODO: turn this into a set of enum variants or something instead
}

impl Color {
    /// Returns the color's internal identifier.
    pub fn id(&self) -> ColorId {
        self.id
    }

    /// Returns the localized name of this color (dye).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the colors base RGB values.
    pub fn base_rgb(&self) -> &[u64; 3] {
        &self.base_rgb
    }

    /// Returns the Color's specific details when applied to cloth (light) armor.
    pub fn cloth(&self) -> &ColorDetails {
        &self.cloth
    }

    /// Returns the Color's specific details when applied to leather (medium) armor.
    pub fn leather(&self) -> &ColorDetails {
        &self.leather
    }

    /// Returns the Color's specific details when applied to metal (heavy) armor.
    pub fn metal(&self) -> &ColorDetails {
        &self.metal
    }

    /// Returns the item identifier of the dye that produces this Color.
    pub fn item(&self) -> ItemId {
        self.item
    }

    /// Returns the list of categories this dye applies to. These represent the hue, material and
    /// rarity of the dye.
    ///
    /// See: https://wiki.guildwars2.com/wiki/API:2/colors for more information
    pub fn categories(&self) -> &[String] {
        self.categories.as_slice()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ColorDetails {
    brightness: i64,
    contrast: f64,
    hue: i64,
    saturation: f64,
    lightness: f64,
    rgb: [u64; 3],
}

impl ColorDetails {
    /// Returns the associated brightness value.
    pub fn brightness(&self) -> i64 {
        self.brightness
    }

    /// Returns the associated contrast value.
    pub fn contrast(&self) -> f64 {
        self.contrast
    }

    /// Returns the associated hue value.
    pub fn hue(&self) -> i64 {
        self.hue
    }

    /// Returns the associated saturation value
    pub fn saturation(&self) -> f64 {
        self.saturation
    }

    /// Returns the associated lightness value.
    pub fn lightness(&self) -> f64 {
        self.lightness
    }

    /// Returns the associated RGB array.
    pub fn rgb(&self) -> &[u64; 3] {
        &self.rgb
    }
}
