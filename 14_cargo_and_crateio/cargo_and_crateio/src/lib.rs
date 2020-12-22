//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor; // 再exportによりクレートユーザーが階層構造を意識せずにenumを利用できる
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor { Red, Yellow, Blue }
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor { Orange, Green, Purple }
}

pub mod utils {
    use crate::kinds::*;
    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

/// ## Semantics
/// Adds one to the given number.
/// ## Examples
/// 
/// ```
/// let five = 5;
/// assert_eq!(6, cargo_and_crateio::add_one(5));
///``````
pub fn add_one(x: i32) -> i32 {
    x + 1
}