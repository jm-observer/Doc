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
    let delta = delta_do_insert_buffer();
    debug!("{delta:?}");
    let rs = resolve_delta_rs(rope, &delta);
    log::debug!("{rs:?}");

    Ok(())
}

fn delta_do_insert_buffer() -> RopeDelta {
    RopeDelta {
        els: vec![DeltaElement::<RopeInfo>::Copy(0, 117),
                  DeltaElement::<RopeInfo>::Insert(Node::from_leaf("m".to_string())), DeltaElement::<RopeInfo>::Copy(117, 461),],
        base_len: 461,
    }
}