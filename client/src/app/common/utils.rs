use crate::filter_system::ParseError;

pub trait FilterErrorHandler {
    fn handle_filter_error(&mut self, error: Option<ParseError>);
    fn is_filter_valid(&self) -> bool;
}

pub fn parse_ip_filter(value: &str) -> Result<String, ParseError> {
    if !value.contains('.') {
        return Err(ParseError::InvalidSyntax(
            "Invalid IP address format".into(),
        ));
    }
    Ok(value.to_lowercase())
}
