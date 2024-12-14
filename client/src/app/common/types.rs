pub struct TableConfig {
    pub row_height: f32,
    pub header_height: f32,
    pub space_after_filter: f32,
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
