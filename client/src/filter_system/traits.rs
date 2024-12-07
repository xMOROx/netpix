use crate::filter_system::ParseError;

pub trait FilterParser: Sized {
    fn parse_filter_value(prefix: &str, value: &str) -> Result<Self, ParseError>;
}

pub trait FilterExpression<'a> {
    type Context;
    fn matches(&self, ctx: &Self::Context) -> bool;
}

pub trait FilterCombinator<'a>: Sized {
    fn and(left: Self, right: Self) -> Self;
    fn or(left: Self, right: Self) -> Self;
}

pub trait CommonFilterParser: FilterParser + Sized {
    fn not(expr: Self) -> Self;
}
