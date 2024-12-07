use floem::kurbo::Point;
use floem::views::editor::visual_line::hit_position_aff;
use floem_editor_core::cursor::{CursorAffinity, CursorMode};
use log::info;
use lsp_types::Position;
use doc::lines::fold::{FoldingDisplayItem, FoldingDisplayType};
use crate::lines_util::_init_lines;

mod lines_util;

#[test]
fn test() {
    custom_utils::logger::logger_stdout_debug();
    let (lines, _) = _init_lines(None);
    // let (vl, offset, last_char) = lines_util.visual_line_of_offset(70, CursorAffinity::Backward);
    // info!("{vl:?}, offset={offset}, {last_char}");
    // |fn main() {n
    // |012345678901
    // info!("{:?} {:?}", lines_util.buffer.line_content(0), lines_util.buffer.line_ending());
    // let (vl, offset_of_visual, offset_folded, last_char) = lines_util.visual_line_of_offset(11, CursorAffinity::Backward);


    // let (offset_of_buffer, is_inside) = lines_util.buffer_offset_of_point(&CursorMode::Normal(0), Point::new(72.2, 58.1));
    // info!("offset_of_buffer={offset_of_buffer},is_inside={is_inside}");
    //below end of buffer
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(138.1, 417.1));
        assert_eq!(offset_of_buffer, 139);
        assert_eq!(is_inside, false);
    }
    //f
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(1.1, 33.1));
        assert_eq!(offset_of_buffer, 0);
        assert_eq!(is_inside, true);
    }
    //end of first line
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(163.1, 33.1));
        assert_eq!(offset_of_buffer, 11);
        assert_eq!(is_inside, false);
    }


    let (vl, offset_of_visual, offset_folded, last_char) = lines.visual_line_of_offset(0, CursorAffinity::Backward);
    info!("offset_of_visual={offset_of_visual},offset_folded={offset_folded}, {last_char}, {vl:?}");

    info!("hit_position_aff {:?}", hit_position_aff(
            &vl.text_layout.text,
            offset_folded,
            true,
        )
            .point);
}

#[test]
fn test_folded_line_1() {
    let (_lines, _) = _init_lines(Some(vec![FoldingDisplayItem {
        position: Position {
            line: 1,
            character: 12,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldStart,
    }]));
}

#[test]
fn test_folded_line_1_5() {
    let (_lines, _) = _init_lines(Some(vec![FoldingDisplayItem {
        position: Position {
            line: 1,
            character: 12,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldStart,
    }, FoldingDisplayItem {
        position: Position {
            line: 5,
            character: 5,
        },
        y: 0,
        ty: FoldingDisplayType::UnfoldEnd,
    }]));
}