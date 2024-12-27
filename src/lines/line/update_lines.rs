use crate::lines::{DocLines};
use crate::lines::buffer::rope_text::RopeText;
use crate::lines::delta_compute::{origin_lines_delta, OriginLinesDelta};
use crate::lines::line::OriginLine;

impl DocLines {

    fn init_all_origin_line_new(
        &self,
        lines_delta: &Option<OriginLinesDelta>
    ) -> anyhow::Result<Vec<OriginLine>> {
        let (recompute_first_line, copy_line_start, recompute_line_start, recompute_offset_end, copy_line_end, recompute_last_line) = origin_lines_delta(lines_delta);


        let mut origin_lines = Vec::with_capacity(self.buffer().num_lines());
        let mut start_offset = 0;
        if recompute_first_line {
            let line = self.init_origin_line(0)?;
            origin_lines.push(line);
        }
        if !copy_line_start.is_empty() {
            // (&self.origin_lines[copy_line_start.start..copy_line_start.end]).into_iter().map(|x| {
            //     todo!()
            // });
            // origin_lines.extend(&self.origin_lines[copy_line_start.start..copy_line_start.end]);
        }
        let last_line = self.buffer().last_line();
        for x in recompute_line_start..=last_line {
            let line = self.init_origin_line(x)?;
            todo!()
            // let end_offset = line.end_offset;
            // origin_lines.push(line);
            // if end_offset >= recompute_offset_end {
            //     break;
            // }
        }
        if !copy_line_end.is_empty() {
            // origin_lines.extend(&self.origin_lines[copy_line_end.start..copy_line_end.end]);
        }
        if recompute_first_line {
            origin_lines.push(self.init_origin_line(last_line)?);
        }
        Ok(origin_lines)
    }
}