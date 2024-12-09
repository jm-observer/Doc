use std::path::PathBuf;
use floem::kurbo::{Point, Rect};
use floem_editor_core::cursor::{CursorAffinity, CursorMode};
use crate::lines_util::{_init_code, _init_origin_code};

mod lines_util;

#[test]
fn test_buffer_offset_of_click() {
    custom_utils::logger::logger_stdout_debug();
    let file: PathBuf = "resources/test_code/main.rs".into();
    let (lines, _) = _init_origin_code(_init_code(file));

    //below end of buffer
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(138.1, 417.1));
        assert_eq!(offset_of_buffer, 143);
        assert_eq!(is_inside, false);
    }
    //f
    {
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(1.1, 13.1));
        assert_eq!(offset_of_buffer, 0);
        assert_eq!(is_inside, true);
    }
    //end of first line
    {
        let point = Point::new(163.1, 13.1);
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), point);
        assert_eq!(offset_of_buffer, 15);
        assert_eq!(is_inside, false);
    }
}


#[test]
fn test_buffer_offset_of_click_2() {
    custom_utils::logger::logger_stdout_debug();
    let file: PathBuf = "resources/test_code/main_2.rs".into();
    let (mut lines, _) = _init_origin_code(_init_code(file));

    // scroll 23 line { x0: 0.0, y0: 480.0, x1: 606.8886108398438, y1: 1018.1586303710938 }
    lines.update_viewport(Rect::new(0.0, 480.0, 606.8, 1018.1));
    //below end of buffer
    {
        // single_click (144.00931549072266, 632.1586074829102) new_offset=480
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(186.0, 608.1));
        lines.log();
        assert_eq!(offset_of_buffer, 456);
        assert_eq!(is_inside, false);

        let (_, _, _, _, point, _) = lines.cursor_position_of_buffer_offset(offset_of_buffer, CursorAffinity::Forward);
        assert_eq!(point.unwrap().y, 118.0 + lines.viewport().y0);
    }
}