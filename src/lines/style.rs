use floem::peniko::Color;
use serde::{Deserialize, Serialize};
use crate::lines::delta_compute::Offset;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NewLineStyle {
    pub origin_line:              usize,
    /// 所在行的起始位置
    pub origin_line_offset_start: usize,
    pub len:   usize,
    /// 在整个buffer的起始位置
    pub start_of_buffer:          usize,
    pub end_of_buffer:            usize,
    pub fg_color:                 Color,
    pub folded_line_offset_start: usize,
    pub folded_line_offset_end:   usize /* pub fg_color:
                                         * Option<String>, */
}

impl NewLineStyle {
    pub fn adjust(&mut self, offset: Offset, line_offset: Offset) {
        offset.adjust(&mut self.start_of_buffer);
        offset.adjust(&mut self.end_of_buffer);
        line_offset.adjust(&mut self.origin_line);
    }
}
