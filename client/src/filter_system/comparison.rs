#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonFilter<T> {
    GreaterThan(T),
    GreaterOrEqualThan(T),
    LessThan(T),
    LessOrEqualThan(T),
    Equals(String),
}

impl<T: std::str::FromStr> ComparisonFilter<T> {
    pub fn parse(value: &str) -> Option<Self> {
        if let Some(stripped) = value.strip_prefix('>') {
            if let Some(stripped) = stripped.strip_prefix('=') {
                stripped.trim().parse().ok().map(Self::GreaterOrEqualThan)
            } else {
                stripped.trim().parse().ok().map(Self::GreaterThan)
            }
        } else if let Some(stripped) = value.strip_prefix("<=") {
            stripped.trim().parse().ok().map(Self::LessOrEqualThan)
        } else if let Some(stripped) = value.strip_prefix('<') {
            stripped.trim().parse().ok().map(Self::LessThan)
        } else {
            Some(Self::Equals(value.to_string()))
        }
    }
}
