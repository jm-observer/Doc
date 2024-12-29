use lapce_xi_rope::{RopeDelta, RopeInfo, DeltaElement};
use lapce_xi_rope::tree::Node;
use log::{debug};
use doc::lines::buffer::rope_text::RopeText;
use doc::lines::delta_compute::resolve_delta_rs;
use doc::lines::EditBuffer;
use doc::lines::line::update_lines::{check_origin_folded_lines, check_origin_lines};
use crate::lines_util::{cursor_insert, init_main_2};

mod lines_util;


#[test]
fn test_debug() -> anyhow::Result<()> {
    custom_utils::logger::logger_stdout_debug();
    let mut lines = init_main_2()?;

    let mut cursor = cursor_insert(117, 117);
    let s = "m";
    let mut response = Vec::new();
    let edit = EditBuffer::DoInsertBuffer {
        cursor:   &mut cursor,
        s:        s,
        response: &mut response
    };

    // let rope = lines.buffer().text();
    // debug!("buffer len={} num_lines={}", lines.buffer().len(), lines.buffer().num_lines());
    // lines.check_lines();
    // let delta = delta_do_insert_buffer();
    // let rs = resolve_delta_rs(rope, &delta)?;
    // debug!("{rs:?}");
    // lines.log();
    // debug!("{:?}", _lines.init_all_origin_line_new(&mut None).unwrap());
    // let mut lines_delta = Some(rs);
    let _ = lines.buffer_edit(edit).unwrap();
    lines.check_lines();

    lines.log();
    // let origin_lines  = lines.init_all_origin_line_new(&mut lines_delta)?;
    // check_origin_lines(&origin_lines, lines.buffer().len() + 1);
    // // for line in &origin_lines {
    // //     debug!("{:?}", line);
    // // }
    // let origin_folded_lines  = lines.init_all_origin_folded_line_new(&lines_delta, &origin_lines)?;
    // check_origin_folded_lines(&origin_folded_lines, lines.buffer().len() + 1);


    Ok(())
}

fn delta_do_insert_buffer() -> RopeDelta {
    RopeDelta {
        els: vec![DeltaElement::<RopeInfo>::Copy(0, 117),
                  DeltaElement::<RopeInfo>::Insert(Node::from_leaf("m".to_string())), DeltaElement::<RopeInfo>::Copy(117, 461),],
        base_len: 461,
    }
}