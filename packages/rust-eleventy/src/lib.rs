/// An absolutely useless function that always returns 110.
///
/// eleventy, from Old English hundendleftig (also spelled hund-endleofantig,
/// hund-endlyftig, and hund-ælleftig).
///
/// Originally derived from Tolkien's Lord of the Rings trilogy.
/// Bilbo Baggin's eleventy-first birth day was the setting of the opening chapter.
/// Since used by people all over the world.
///
/// ```
/// let result = eleventy::eleventy();
/// assert_eq!(result, 110);
/// ```
///
/// ```
/// let result = eleventy::eleventy();
/// assert_ne!(result, 42);
/// ```
pub const fn eleventy() -> u8 {
    110
}
