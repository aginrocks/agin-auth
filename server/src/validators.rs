use validator::ValidationError;

pub fn is_valid_username(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

pub fn username_validator(s: &str) -> Result<(), ValidationError> {
    if !is_valid_username(s) {
        return Err(ValidationError::new("invalid_format"));
    }

    Ok(())
}
