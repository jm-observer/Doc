#![allow(unused_imports, dead_code, unused_mut)]
use std::path::PathBuf;
use std::sync::atomic;
use floem::kurbo::{Point, Rect};
use floem::reactive::SignalUpdate;
use floem_editor_core::command::EditCommand;
use floem_editor_core::register::Register;
use lapce_xi_rope::{DeltaElement, Interval, RopeInfo};
use lapce_xi_rope::spans::SpansBuilder;
use log::info;
use doc::lines::buffer::rope_text::RopeText;
use doc::lines::cursor::{Cursor, CursorAffinity, CursorMode};
use doc::lines::selection::Selection;
use crate::lines_util::{init_empty, init_main, init_main_2, init_semantic_2};

mod lines_util;


#[test]
fn test_performance() {
    custom_utils::logger::logger_stdout_debug();
    let file: PathBuf = "resources/test_code/empty.rs".into();
    let editor: PathBuf = "resources/test_code/editor.rs".into();
    let editor_code = std::fs::read_to_string(editor).unwrap();
    let mut lines = init_empty();

    lines.init_buffer(editor_code.into());
}

#[test]
fn test_semantic_2() {
    custom_utils::logger::logger_stdout_debug();
    let lines = init_main_2();

}

#[test]
fn test_buffer_offset_of_click() {
    custom_utils::logger::logger_stdout_debug();
    // let file: PathBuf = "resources/test_code/main.rs".into();
    let lines = init_main();
    lines.log();

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
    let mut lines = init_main_2();

    // scroll 23 line { x0: 0.0, y0: 480.0, x1: 606.8886108398438, y1: 1018.1586303710938 }
    lines.update_viewport_by_scroll(Rect::new(0.0, 480.0, 606.8, 1018.1));
    //below end of buffer
    {
        // single_click (144.00931549072266, 632.1586074829102) new_offset=480
        let (offset_of_buffer, is_inside) = lines.buffer_offset_of_click(&CursorMode::Normal(0), Point::new(186.0, 608.1));
        lines.log();
        assert_eq!(offset_of_buffer, 456);
        assert_eq!(is_inside, false);

        let (_, _, _, _, point, _, _) = lines.cursor_position_of_buffer_offset(offset_of_buffer, CursorAffinity::Forward);
        assert_eq!(point.unwrap().y, 118.0 + lines.viewport().y0);
    }
}

#[test]
fn test_buffer_edit() {
    custom_utils::logger::logger_stdout_debug();
    let mut lines = init_main_2();

    // info!("enable_error_lens {}", lines.config.enable_error_lens);
    // let start_offset = lines.buffer.offset_of_line(10);
    // let end_offset = lines.buffer.offset_of_line(11);
    // let styles = lines.get_line_diagnostic_styles(start_offset, end_offset, &mut None, 0);
    // info!("{:?}", styles);
    // lines.log();

    info!("{:?} {:?} {:?} {:?}", lines.buffer.char_at_offset(181), lines.buffer.char_at_offset(182), lines.buffer.char_at_offset(183), lines.buffer.char_at_offset(184));
    let mut cursor = cursor_insert();
    let mut register = Register::default();
    let deltas = lines.do_edit_buffer(&mut cursor, &EditCommand::InsertNewLine,false, &mut register, true,);

    let mut change_start = usize::MAX;
    let mut change_end = 0;
    for (rope, delta, inval) in &deltas {
        let mut single_change_start = 0;
        let mut single_change_end = rope.len();
        if let Some(first) = delta.els.first() {
            match first {
                DeltaElement::Copy(start, end) => {
                    if *start == 0 {
                        single_change_start = *end;
                    }
                }
                DeltaElement::Insert(_) => {}
            }
        }
        if let Some(last) = delta.els.last() {
            match last {
                DeltaElement::Copy(start, end) => {
                    if *end == single_change_end {
                        single_change_end = *start;
                    }
                }
                DeltaElement::Insert(_) => {}
            }
        }
        if single_change_start < change_start {
            change_start = single_change_start
        }
        if single_change_end > change_end {
            change_end = single_change_end
        }
    }

}


fn cursor_insert() -> Cursor {
    let mode = CursorMode::Insert(Selection::region(139, 139));
    Cursor::new(mode, None, None)
}

fn cursor_normal() -> Cursor {
    let mode = CursorMode::Normal(183);
    Cursor::new(mode, None, None)
}