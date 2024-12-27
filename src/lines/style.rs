use floem::peniko::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewLineStyle {
    pub origin_line:              usize,
    pub origin_line_offset_start: usize,
    pub len:   usize,
    pub start_of_buffer:          usize,
    pub end_of_buffer:            usize,
    pub fg_color:                 Color,
    pub folded_line_offset_start: usize,
    pub folded_line_offset_end:   usize /* pub fg_color:
                                         * Option<String>, */
}
