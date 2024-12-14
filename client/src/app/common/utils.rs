use crate::filter_system::ParseError;

pub fn parse_ip_filter(value: &str) -> Result<String, ParseError> {
    if !value.contains('.') {
        return Err(ParseError::InvalidSyntax(
            "Invalid IP address format".into(),
        ));
    }
    Ok(value.to_lowercase())
}
