use floem::kurbo::Point;
use floem_editor_core::cursor::{CursorAffinity, CursorMode};
use crate::lines_util::{_init_origin_code};

mod lines_util;

#[test]
fn test_buffer_offset_of_click() {
    custom_utils::logger::logger_stdout_debug();
    let (lines, _) = _init_origin_code();

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
fn test_next_visual_line() {
    custom_utils::logger::logger_stdout_debug();
    let (lines, _) = _init_origin_code();

    //move to last line
    {
        let (visual_line, _, _) = lines.next_visual_line(8, 9 , CursorAffinity::Backward);
        assert_eq!(visual_line.line_index, 9);
    }
}