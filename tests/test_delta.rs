use floem::views::editor::core::command::EditCommand;
use floem::views::editor::core::register::Register;
use lapce_xi_rope::{RopeDelta, RopeInfo, DeltaElement};
use lapce_xi_rope::tree::Node;
use doc::lines::EditBuffer;
use crate::lines_util::{cursor_insert, init_main_2};

mod lines_util;


#[test]
fn test_do_insert() -> anyhow::Result<()> {
    custom_utils::logger::logger_stdout_debug();

    {
        let mut lines = init_main_2()?;
        let mut cursor = cursor_insert(117, 117);
        let s = "m";
        let mut response = Vec::new();
        let edit = EditBuffer::DoInsertBuffer {
            cursor: &mut cursor,
            s: s,
            response: &mut response,
        };
        let _ = lines.buffer_edit(edit).unwrap();
        if lines.check_lines() {
            lines.log();
        }
    }

    Ok(())
}

#[test]
fn test_insert_new_line() -> anyhow::Result<()> {
    custom_utils::logger::logger_stdout_debug();
    {
        let mut lines = init_main_2()?;
        let mut cursor = cursor_insert(117, 117);
        let cmd = EditCommand::InsertNewLine;
        let mut register = Register::default();
        let smart_tab = true;
        let mut response = Vec::new();

        let edit = EditBuffer::DoEditBuffer {
            cursor:    &mut cursor,
            cmd:       &cmd,
            modal:     false,
            register:  &mut register,
            smart_tab,
            response: &mut response,
        };
        let _ = lines.buffer_edit(edit).unwrap();
        if !lines.check_lines() {
            lines.log();
        }
    }


    Ok(())
}

fn _delta_do_insert_buffer() -> RopeDelta {
    RopeDelta {
        els: vec![DeltaElement::<RopeInfo>::Copy(0, 117),
                  DeltaElement::<RopeInfo>::Insert(Node::from_leaf("m".to_string())), DeltaElement::<RopeInfo>::Copy(117, 461),],
        base_len: 461,
    }
}