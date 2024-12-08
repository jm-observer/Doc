use floem::kurbo::Point;
use floem_editor_core::cursor::{CursorAffinity, CursorMode};
use log::debug;
use lsp_types::Position;
use doc::lines::fold::{FoldingDisplayItem, FoldingDisplayType};
use crate::lines_util::{_init_folded_code_v1, _init_folded_code_v2, check_line_final_col, check_lines_col};

mod lines_util;

#[test]
fn test_folded_line_1() {
    custom_utils::logger::logger_stdout_debug();
    let (_lines, _) = _init_folded_code_v1();
    {
        let line2 = &_lines.visual_lines[1];
        check_lines_col(
            &line2.text_layout.phantom_text.text,
            line2.text_layout.phantom_text.final_text_len,
            "    if true {\r\n    } else {\r\n",
            "    if true {...} else {\r\n",
        );
        check_line_final_col(&line2.text_layout.phantom_text, "    if true {...} else {\r\n");
    }
    {
        let let_line = &_lines.visual_lines[4];
        debug!("{:?}", let_line);
        let expect_str = "    let a: A  = A;\r\n";

        check_lines_col(
            &let_line.text_layout.phantom_text.text,
            let_line.text_layout.phantom_text.final_text_len,
            "    let a = A;\r\n",
            expect_str,
        );
        check_line_final_col(&let_line.text_layout.phantom_text, expect_str);
    }
}

#[test]
fn test_folded_line_1_5() {
    let (_lines, _) = _init_folded_code_v2();
    {
        let line2 = &_lines.visual_lines[1];
        check_lines_col(
            &line2.text_layout.phantom_text.text,
            line2.text_layout.phantom_text.final_text_len,
            "    if true {\r\n    } else {\r\n    }\r\n",
            "    if true {...} else {...}\r\n",
        );
        check_line_final_col(&line2.text_layout.phantom_text, "    if true {...} else {...}\r\n");
    }
}