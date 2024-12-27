use crate::lines::{DocLines};
use crate::lines::buffer::rope_text::RopeText;
use crate::lines::delta_compute::{origin_lines_delta, OriginLinesDelta};
use crate::lines::line::OriginLine;

impl DocLines {

    fn init_all_origin_line_new(
        &self,
        lines_delta: &Option<OriginLinesDelta>
    ) -> anyhow::Result<Vec<OriginLine>> {
        let (recompute_first_line, copy_line_start_offset, copy_line_start, recompute_line_start, recompute_offset_end, copy_line_end, copy_line_end_offset, recompute_last_line) = origin_lines_delta(lines_delta);

        let mut origin_lines = Vec::with_capacity(self.buffer().num_lines());
        let mut line_index = 0;
        if recompute_first_line {
            let line = self.init_origin_line(0)?;
            origin_lines.push(line);
            line_index += 1;
        }
        if !copy_line_start.is_empty() {
            origin_lines.extend((&self.origin_lines[copy_line_start.start..copy_line_start.end]).into_iter().map(|x| {
                let mut x = x.clone();
                copy_line_start_offset.adjust(&mut x.start_offset);
                x.line_index = line_index;
                x
            }));
        }
        let last_line = self.buffer().last_line();
        for x in recompute_line_start..=last_line {
            let line = self.init_origin_line(x)?;
            let end = line.start_offset + line.len;
            origin_lines.push(line);
            line_index += 1;
            if end >= recompute_offset_end {
                break;
            }
        }
        if !copy_line_end.is_empty() {
            origin_lines.extend((&self.origin_lines[copy_line_end.start..copy_line_end.end]).into_iter().map(|x| {
                let mut x = x.clone();
                copy_line_end_offset.adjust(&mut x.start_offset);
                x.line_index = line_index;
                x
            }));
        }
        if recompute_last_line {
            origin_lines.push(self.init_origin_line(last_line)?);
        }
        Ok(origin_lines)
    }
}