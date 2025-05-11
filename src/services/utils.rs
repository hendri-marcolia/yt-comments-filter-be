use blake3;
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;
use lazy_static::lazy_static;

lazy_static! {
    static ref FANCY_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        // Map for chars ᴍᴀɴᴅᴀʟɪᴋᴀ
        let pairs = [
            // Fullwidth or Enclosed Alphanumerics
            ('🅰', 'A'), ('🅱', 'B'), ('🅲', 'C'), ('🅳', 'D'), ('🅴', 'E'), ('🅵', 'F'),
            ('🅶', 'G'), ('🅷', 'H'), ('🅸', 'I'), ('🅹', 'J'), ('🅺', 'K'), ('🅻', 'L'),
            ('🅼', 'M'), ('🅽', 'N'), ('🅾', 'O'), ('🅿', 'P'), ('🆀', 'Q'), ('🆁', 'R'),
            ('🆂', 'S'), ('🆃', 'T'), ('🆄', 'U'), ('🆅', 'V'), ('🆆', 'W'), ('🆇', 'X'),
            ('🆈', 'Y'), ('🆉', 'Z'),

            // Bold/Monospace
            ('𝗔', 'A'), ('𝗕', 'B'), ('𝗖', 'C'), ('𝗗', 'D'), ('𝗘', 'E'), ('𝗙', 'F'),
            ('𝗚', 'G'), ('𝗛', 'H'), ('𝗜', 'I'), ('𝗝', 'J'), ('𝗞', 'K'), ('𝗟', 'L'),
            ('𝗠', 'M'), ('𝗡', 'N'), ('𝗢', 'O'), ('𝗣', 'P'), ('𝗤', 'Q'), ('𝗥', 'R'),
            ('𝗦', 'S'), ('𝗧', 'T'), ('𝗨', 'U'), ('𝗩', 'V'), ('𝗪', 'W'), ('𝗫', 'X'),
            ('𝗬', 'Y'), ('𝗭', 'Z'),

            ('𝙰', 'A'), ('𝙱', 'B'), ('𝙲', 'C'), ('𝙳', 'D'), ('𝙴', 'E'), ('𝙵', 'F'),
            ('𝙶', 'G'), ('𝙷', 'H'), ('𝙸', 'I'), ('𝙹', 'J'), ('𝙺', 'K'), ('𝙻', 'L'),
            ('𝙼', 'M'), ('𝙽', 'N'), ('𝙾', 'O'), ('𝙿', 'P'), ('𝚀', 'Q'), ('𝚁', 'R'),
            ('𝚂', 'S'), ('𝚃', 'T'), ('𝚄', 'U'), ('𝚅', 'V'), ('𝚆', 'W'), ('𝚇', 'X'),
            ('𝚈', 'Y'), ('𝚉', 'Z'),

            // Stylized numerals
            ('𝟎', '0'), ('𝟏', '1'), ('𝟐', '2'), ('𝟑', '3'), ('𝟒', '4'),
            ('𝟓', '5'), ('𝟔', '6'), ('𝟕', '7'), ('𝟖', '8'),

            // Armenian etc.
            ('է', 't'), ('օ', 'o'),

            // Enclosed alphanumerics
            ('⒜', 'a'), ('⒝', 'b'), ('⒞', 'c'), ('⒟', 'd'), ('⒠', 'e'), ('⒡', 'f'),
            ('⒢', 'g'), ('⒣', 'h'), ('⒤', 'i'), ('⒥', 'j'), ('⒦', 'k'), ('⒧', 'l'),
            ('⒨', 'm'), ('⒩', 'n'), ('⒪', 'o'), ('⒫', 'p'), ('⒬', 'q'), ('⒭', 'r'),
            ('⒮', 's'), ('⒯', 't'), ('⒰', 'u'), ('⒱', 'v'), ('⒲', 'w'), ('⒳', 'x'),
            ('⒴', 'y'), ('⒵', 'z'),

            ('ⓐ', 'a'), ('ⓑ', 'b'), ('ⓒ', 'c'), ('ⓓ', 'd'), ('ⓔ', 'e'), ('ⓕ', 'f'),
            ('ⓖ', 'g'), ('ⓗ', 'h'), ('ⓘ', 'i'), ('ⓙ', 'j'), ('ⓚ', 'k'), ('ⓛ', 'l'),
            ('ⓜ', 'm'), ('ⓝ', 'n'), ('ⓞ', 'o'), ('ⓟ', 'p'), ('ⓠ', 'q'), ('ⓡ', 'r'),
            ('ⓢ', 's'), ('ⓣ', 't'), ('ⓤ', 'u'), ('ⓥ', 'v'), ('ⓦ', 'w'), ('ⓧ', 'x'),
            ('ⓨ', 'y'), ('ⓩ', 'z'),
        ];

        for (k, v) in pairs {
            m.insert(k, v);
        }
        m
    };
}

/// Normalize and hash a YouTube comment using Blake3.
/// Trims whitespace and lowercases the input before hashing.
/// Returns a lowercase hex digest (64 characters).
/// This function normalizes the input string by removing diacritics and converting
/// fancy characters to their ASCII equivalents.
/// It also converts the string to lowercase and filters out non-alphanumeric characters.
pub fn normalize_fancy_text(input: &str) -> String {
    input
        .chars()
        .map(|c| *FANCY_MAP.get(&c).unwrap_or(&c))          // Fancy char normalization
        .collect::<String>()
        .nfkd()                                              // Unicode decomposition
        .filter(|c| !('\u{0300}'..='\u{036F}').contains(c)) // Removes combining diacritics
        .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace())              // Remove non-alphanumerics
        .flat_map(|c| c.to_lowercase())                     // Lowercase output
        .collect()
}

/// Normalize and hash a YouTube comment using Blake3.
/// Trims whitespace and lowercases the input before hashing.
/// Returns a lowercase hex digest (64 characters).
pub fn hash_comment(comment: &str) -> String {
    let normalized = normalize_fancy_text(comment);
    blake3::hash(normalized.as_bytes()).to_hex().to_string()
}
