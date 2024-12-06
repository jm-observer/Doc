use lapce_xi_rope::Interval;
use std::sync::Arc;
use floem::views::editor::layout::TextLayoutLine;
use floem::views::editor::visual_line::{RVLine, VLine, VLineInfo};
use std::fmt::{Debug, Formatter};
use floem_editor_core::line_ending::LineEnding;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct OriginLine {
    pub line_index: usize,
    pub start_offset: usize,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct OriginFoldedLine {
    pub line_index: usize,
    // [origin_line_start..origin_line_end]
    pub origin_line_start: usize,
    pub origin_line_end: usize,
    pub origin_interval: Interval,
    pub text_layout: Arc<TextLayoutLine>,
}

#[allow(dead_code)]
impl OriginFoldedLine {
    fn final_offset_of_visual_line(
        &self,
        sub_line_index: usize,
        line_offset: usize,
    ) -> usize {
        let final_offset =
            self.text_layout.text.line_layout().iter().enumerate().fold(
                line_offset,
                |mut offset, (index, layout)| {
                    if sub_line_index < index {
                        offset += layout.glyphs.len();
                    }
                    offset
                },
            );
        let (_orgin_line, _offset_of_line, offset_of_buffer) = self
            .text_layout
            .phantom_text
            .cursor_position_of_final_col(final_offset);
        offset_of_buffer
    }

    /// 求原始的行的偏移，最终出现在第几个视觉行，以及在视觉行的偏移位置，以及合并行的偏移位置
    pub(crate) fn visual_line_of_line_and_offset(&self, origin_line: usize, offset:usize) -> (usize, usize, usize) {
        let final_offset = self
            .text_layout
            .phantom_text
            .final_col_of_col(origin_line, offset, true);
        let (sub_line, offset_of_visual) = self.visual_line_of_final_offset(final_offset);
        (sub_line, offset_of_visual, final_offset)
    }

    /// 求最终的行偏移出现在第几个视觉行，以及在视觉行的偏移位置
    fn visual_line_of_final_offset(&self, final_offset:usize) -> (usize, usize) {
        // 空行时，会出现==的情况
        if final_offset > self.len() {
            panic!("final_offset={final_offset} >= {}", self.len())
        }
        let folded_line_layout = self.text_layout.text.line_layout();
        if folded_line_layout.len() == 1 {
            return (0, final_offset);
        }
        let mut sub_line_index = folded_line_layout.len() - 1;
        let mut final_offset_line = final_offset;
        // let mut last_char = false;

        for (index, sub_line) in folded_line_layout.iter().enumerate() {
            if final_offset_line <= sub_line.glyphs.len() {
                sub_line_index = index;
                // last_char = final_offset == sub_line.glyphs.len() - self.text_layout.text.;
                break;
            } else {
                final_offset_line -= sub_line.glyphs.len();
            }
        }
        (sub_line_index, final_offset_line)
    }

    fn len(&self) -> usize {
        self.text_layout.text.line().text().len()
    }

    pub(crate) fn len_without_rn(&self, ending: LineEnding) -> usize {
        self.len().max(ending.len()) - ending.len()
    }
}

impl Debug for OriginFoldedLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OriginFoldedLine line_index={} origin_line_start={} origin_line_end={} origin_interval={}  phantom_text={:?}",
               self.line_index, self.origin_line_start, self.origin_line_end, self.origin_interval, self.text_layout.phantom_text)
    }
}

#[derive(Clone)]
pub struct VisualLine {
    pub line_index: usize,
    pub origin_interval: Interval,
    pub visual_interval: Interval,
    pub origin_line: usize,
    pub origin_folded_line: usize,
    pub origin_folded_line_sub_index: usize,
    pub text_layout: Arc<TextLayoutLine>,
}

impl Debug for VisualLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VisualLine")
            .field("line_index", &self.line_index)
            .field("origin_interval", &self.origin_interval)
            .field("visual_interval", &self.visual_interval)
            .field("origin_line", &self.origin_line)
            .field("origin_folded_line", &self.origin_folded_line)
            .field(
                "origin_folded_line_sub_index",
                &self.origin_folded_line_sub_index,
            )
            .field("text_layout", &self.text_layout.text.line().layout_opt())
            .field("phantom_text", &self.text_layout.phantom_text)
            .finish()
    }
}

impl VisualLine {
    pub fn rvline(&self) -> RVLine {
        RVLine {
            line: self.origin_folded_line,
            line_index: self.origin_folded_line_sub_index,
        }
    }

    pub fn vline(&self) -> VLine {
        VLine(self.line_index)
    }

    pub fn vline_info(&self) -> VLineInfo {
        let rvline = self.rvline();
        let vline = self.vline();
        let interval = self.origin_interval;
        // todo?
        let origin_line = self.origin_folded_line;
        VLineInfo {
            interval,
            rvline,
            origin_line,
            vline,
        }
    }

    // 行号
    pub fn line_number(
        &self,
        show_relative: bool,
        current_number: Option<usize>,
    ) -> Option<usize> {
        if self.origin_folded_line_sub_index == 0 {
            let line_number = self.origin_line + 1;
            Some(if show_relative {
                if let Some(current_number) = current_number {
                    if line_number == current_number {
                        line_number
                    } else {
                        line_number.abs_diff(current_number)
                    }
                } else {
                    line_number
                }
            } else {
                line_number
            })
        } else {
            None
        }
    }
}

impl From<&VisualLine> for RVLine {
    fn from(value: &VisualLine) -> Self {
        value.rvline()
    }
}

impl From<&VisualLine> for VLine {
    fn from(value: &VisualLine) -> Self {
        value.vline()
    }
}