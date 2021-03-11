use crate::AError;
use boyer_moore_magiclen::BMByte;

pub fn search_find_match(pattern_to_match: &str, text: &str) -> Result<bool, AError> {
    let ptm = BMByte::from(pattern_to_match);

    if ptm.is_none() {
        return Err(AError::msg("Invalid Pattern"));
    };

    if ptm.unwrap().find_first_in(text).is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}
