use egui::{Color32, Vec2};

// UI Layout constants
pub const FILTER_ERROR_WINDOW_OFFSET: Vec2 = Vec2::new(-5.0, -30.0);
pub const HELP_WINDOW_MIN_WIDTH: f32 = 600.0;
pub const VERTICAL_SPACING: f32 = 10.0;

// UI Element labels and texts
pub const FILTER_HINT_TEXT: &str = "Filter expression...";
pub const HELP_BUTTON_TEXT: &str = "ℹ Help";
pub const HELP_BUTTON_TOOLTIP: &str = "Show filter syntax help";
pub const FILTER_ERROR_TITLE: &str = "Filter Error";
pub const FILTER_HELP_TITLE: &str = "Filter Syntax Help";
pub const BASIC_FILTERS_TITLE: &str = "Basic Filters:";
pub const LOGICAL_OPERATORS_TITLE: &str = "Logical Operators:";
pub const EXAMPLES_TITLE: &str = "Examples:";
pub const CLOSE_BUTTON_TEXT: &str = "Close";

// Logical operator descriptions
pub const LOGICAL_OPERATORS: [(&str, &str); 4] = [
    ("AND", "Combine conditions (all must match)"),
    ("OR", "Alternative conditions (any must match)"),
    ("NOT", "Negate condition"),
    ("()", "Group conditions"),
];

// Header styling constants
pub const HEADER_TEXT_SIZE: f32 = 20.0;
pub const SECTION_HEADER_TEXT_SIZE: f32 = 16.0;
pub const HEADER_BOTTOM_MARGIN: f32 = 15.0;
pub const SECTION_BOTTOM_MARGIN: f32 = 8.0;
pub const SECTION_HEADER_COLOR: Color32 = Color32::from_rgb(153, 109, 0); // Dark orange
pub const SOURCE_KEY: &str = "source";
pub const TAB_KEY: &str = "tab";
