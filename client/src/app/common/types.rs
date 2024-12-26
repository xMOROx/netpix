use std::fmt::Display;

pub struct TableConfig {
    pub row_height: f32,
    pub header_height: f32,
    pub space_after_filter: f32,
}

impl TableConfig {
    pub fn new(row_height: f32, header_height: f32, space_after_filter: f32) -> Self {
        Self {
            row_height,
            header_height,
            space_after_filter,
        }
    }
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            row_height: 25.0,
            header_height: 30.0,
            space_after_filter: 5.0,
        }
    }
}
