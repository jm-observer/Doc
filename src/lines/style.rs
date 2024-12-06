use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewLineStyle {
    pub origin_line: usize,
    pub origin_line_offset_start: usize,
    pub origin_line_offset_end: usize,
    pub fg_color: Option<String>,
}
