use std::fmt as StdFmt;

/// Represents valid language options, for endpoints that support it. It corresponds to the GET
/// parameter `lang` in request URIs.
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Locale {
    /// English.
    EN,
    /// Spanish.
    ES,
    /// German.
    DE,
    /// French.
    FR,
    /// Chinese.
    ZH,
}

impl Locale {
    /// Converts itself into the appropriate string representation.
    pub fn to_str(&self) -> &str {
        match *self {
            Locale::EN => "en",
            Locale::ES => "es",
            Locale::DE => "de",
            Locale::FR => "fr",
            Locale::ZH => "zh",
        }
    }
}

/// Implement the standard library's `Display` trait, so that we can use `Locale`s more easily in
/// formatted strings.
impl StdFmt::Display for Locale {
    fn fmt(&self, f: &mut StdFmt::Formatter) -> StdFmt::Result {
        write!(f, "{}", self.to_str())
    }
}
