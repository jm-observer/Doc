use floem::kurbo::Point;
use floem_editor_core::cursor::{CursorMode};
use lsp_types::Position;
use doc::lines::fold::{FoldingDisplayItem, FoldingDisplayType};
use crate::lines_util::_init_lines;

mod lines_util;

#[test]
fn test() {
    custom_utils::logger::logger_stdout_debug();
    let (lines, _) = _init_lines(None);

    //below end of buffer
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(138.1, 417.1));
        assert_eq!(offset_of_buffer, 143);
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
        let point = Point::new(163.1, 33.1);
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), point);
        assert_eq!(offset_of_buffer, 15);
        assert_eq!(is_inside, false);
    }


}

#[test]
fn test_folded_line_1() {
    custom_utils::logger::logger_stdout_debug();
    let (lines, _) = _init_lines(Some(vec![FoldingDisplayItem {
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