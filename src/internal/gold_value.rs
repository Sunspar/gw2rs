use ::std::convert::From as StdFrom;
use ::std::fmt::Display as StdDisplay;
use ::std::fmt::Formatter as StdFormatter;
use ::std::fmt::Result as StdFmtResult;
use ::std::ops::Add as StdAdd;
use ::std::ops::Sub as StdSub;

/// `GoldValue`s are a convenience around monetary values provided by the ArenaNet API.
/// In particular, endpoints that return monetary values natively return the number of copper coins
/// that represents that cost. `GoldValue` is a convenience to convert that amount into gold, silver,
/// and copper coins.
///
/// It implements functionality for the add abd subtract operators. It also implements From for
/// both i64 and u64. Smaller integer sizes may case safely up to their 64bit equivalents.
///
/// # Implementation Details
/// We use f64's to store the values, as that makes supporting non-integral mathematical operations on the underlying
/// copper values much easier. This carries with it the usual issues surrounding decimal precision and rounding, but
/// it should be acceptable for the following reasons:
///
/// 1. f64's are IEEE-754 doubles, which affords us that all integer values less than 2^53 are perfectly representable
/// 2. Money in the game is not actually a fractional item -- copper fractions appear to always round down
/// 3. As you get larger the precision for the decimal part fades, but this is fine because of manual rounding and 2)
/// 4. The maximum gold carryable is 200_000 Gold -- which when converted to copper is 2_000_000_000 Copper, which is
///    -well- below the threshold at which our calculations can start to get inaccurate.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GoldValue {
    /// The number of copper coins.
    coins: f64,
}

impl GoldValue {
    /// Creates a new GoldValue out of its individual components.
    /// Floor's the incoming copper value to be consistent with the API's definition of copper values as integers.
    pub fn new(copper_value: f64) -> GoldValue {
        GoldValue {
            coins: copper_value.floor(),
        }
    }

    /// Returns the underlying u64 that represents this GoldValue's coins value. In game, this represents the number
    /// of copper some value represents.
    pub fn coins(&self) -> f64 {
        self.coins
    }

    /// Whether or not the `GoldValue` represents a positive amount.
    /// In the case of a gold value of `0`, the associated GoldValue may be either positive or negative.
    pub fn positive(&self) -> bool {
        self.coins >= 0f64
    }

    /// Whether or not the `GoldValue` represents a negative amount.
    /// In the case of a gold value of `0`, the associated GoldValue may be either positive or negative.
    pub fn negative(&self) -> bool {
        self.coins < 0f64
    }

    /// The amount of gold coins represented in this `GoldValue`.
    pub fn gold(&self) -> u64 {
        (self.coins.abs() / 10_000f64) as u64
    }

    /// The amount of silver coins represented in this `GoldValue`.
    pub fn silver(&self) -> u64 {
        // xxyyxx
        // take remainder after dividing by 10^4, then divide by 10^2
        ((self.coins.abs() % 10_000f64) / 100f64) as u64
    }

    /// The amount of copper coins represented in this `GoldValue`.
    pub fn copper(&self) -> u64 {
        (self.coins.abs() % 100f64) as u64
    }

    /// Applies the 15% Trading Post tax that applies to all trading post sales.
    ///
    /// 5% is incurred to list the item, and 10% occurs on the sale price of the item once sold.
    /// This method applies the full 15% filter on GoldValue structs by applying it to the underlying copper value.
    ///
    /// Notice: This function is intended to only be used with POSITIVE GoldValue objects -- taxing negative values
    ///         probably won't work as you intend.
    pub fn tax(self) -> Self {
        self.multiply(0.85f64)
    }

    /// Applies the 5% listing cost to a gold value, indicating how much it will cost to list an item for the given
    /// price.
    pub fn trading_post_list_cost(self) -> Self {
        self.multiply(0.05)
    }

    /// Applies the 10% Trading Post tax taken from the sale price of an item once a buyer has been found.
    ///
    /// This tax is subtracted from the final sale price before gold is available for pickup from the Trading post.
    pub fn trading_post_sale_tax(self) -> Self {
        self.multiply(0.10)
    }
    /// Scalar multiplication of the `GoldValue` with a percentage value.
    /// 100% = 100, 8% = 8, etc.
    /// May panic if the multiplication before division of 100 causes an overflow.
    pub fn multiply(self, multiple: f64) -> Self {
        Self::new(self.coins * multiple)
    }
}

/// Implementation of the `Display` trait from the standard library.
impl StdDisplay for GoldValue {
    fn fmt(&self, f: &mut StdFormatter) -> StdFmtResult {
        let mut result_string = String::new();

        if self.negative() {
            result_string.push_str("-");
        }

        let self_gold = self.gold();
        if self_gold > 0 {
            result_string.push_str(&format!("{}g ", self_gold));
        }

        let self_silver = self.silver();
        if self_silver > 0 {
            result_string.push_str(&format!("{}s ", self_silver));
        }

        let self_copper = self.copper();
        if self_copper > 0 {
            result_string.push_str(&format!("{}c ", self_copper));
        }

        write!(f, "{}", result_string.trim())
    }
}

/// Implementation of the standard library's Add trait.
///
/// Provides support for the `+` operator on two `GoldValue`s.
impl StdAdd for GoldValue {
    type Output = GoldValue;

    fn add(self, other: Self) -> Self {
        Self::new(self.coins() + other.coins())
    }
}

/// Implementation of the standard library's Sub trait.
///
/// Provides support for the `-` operator on two `GoldValue`s.
impl StdSub for GoldValue {
    type Output = GoldValue;

    fn sub(self, other: Self) -> Self {
        Self::new(self.coins() - other.coins())
    }
}

/// Implementation of the standard library's From<T> trait for i64 targets.
///
/// Allows for conversion from signed integers into `GoldValue`s.
impl StdFrom<i64> for GoldValue {
    fn from(val: i64) -> Self {
        if val < 0 {
            Self::new(val as f64)
        } else {
            Self::new(val as f64)
        }
    }
}

/// Implementation of the standard library's From<T> trait for u64 targets.
///
/// Allows for conversion from unsigned integers into `GoldValue`s.
impl StdFrom<u64> for GoldValue {
    fn from(val: u64) -> Self {
        Self::new(val as f64)
    }
}

/// Implementation of the standard library's `From<T>` trait for i32 targets.
///
/// Allows for conversion from signed 32-bit integers into `GoldValue`s
impl StdFrom<i32> for GoldValue {
    fn from(val: i32) -> Self {
        if val < 0 {
            Self::new(val as f64)
        } else {
            Self::new(val as f64)
        }
    }
}

/// Implementation of the standard library's `From<T>` trait for u32 targets.
///
/// Allows for conversion from unsigned 32-bit integers into `GoldValue`s
impl StdFrom<u32> for GoldValue {
    fn from(val: u32) -> Self {
        Self::new(val as f64)
    }
}


impl StdFrom<f64> for GoldValue {
    fn from(val: f64) -> Self {
        Self::new(val)
    }
}

impl StdFrom<f32> for GoldValue {
    fn from(val: f32) -> Self {
        Self::new(val as f64)
    }
}