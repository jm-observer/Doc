use lapce_xi_rope::{RopeDelta, RopeInfo, DeltaElement};
use lapce_xi_rope::tree::Node;
use log::debug;
use doc::lines::buffer::rope_text::RopeText;
use doc::lines::delta_compute::resolve_delta_rs;
use crate::lines_util::init_main_2;

mod lines_util;


#[test]
fn test_debug() -> anyhow::Result<()> {
    custom_utils::logger::logger_stdout_debug();
    let _lines = init_main_2()?;
    let rope = _lines.buffer().text();
    debug!("buffer len={} num_lines={}", _lines.buffer().len(), _lines.buffer().num_lines());
    // _lines.log();
    let delta = delta_do_insert_buffer();
    let rs = resolve_delta_rs(rope, &delta).unwrap();
    debug!("{rs:?}");
    debug!("{:?}", _lines.init_all_origin_line_new(&None).unwrap());
    debug!("{:?}", _lines.init_all_origin_line_new(&Some(rs)).unwrap());
    // assert_eq!()

    Ok(())
}

fn delta_do_insert_buffer() -> RopeDelta {
    RopeDelta {
        els: vec![DeltaElement::<RopeInfo>::Copy(0, 117),
                  DeltaElement::<RopeInfo>::Insert(Node::from_leaf("m".to_string())), DeltaElement::<RopeInfo>::Copy(117, 461),],
        base_len: 461,
    }
}