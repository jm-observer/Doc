use floem::reactive::{RwSignal, Scope};
use floem::peniko::Color;
use floem::kurbo::Rect;
use floem::views::editor::EditorStyle;
use crate::lines::buffer::Buffer;
use crate::lines::fold::FoldingDisplayItem;
use crate::lines::screen_lines::ScreenLines;

#[derive(Clone)]
pub struct Signals {
    pub(crate) show_indent_guide: RwSignal<(bool, Color)>,
    pub(crate) viewport: RwSignal<Rect>,
    pub(crate) folding_items_signal: RwSignal<Vec<FoldingDisplayItem>>,
    pub(crate) screen_lines_signal: RwSignal<ScreenLines>,
    pub(crate) buffer_rev: RwSignal<u64>,
    pub(crate) buffer: RwSignal<Buffer>,
    pub(crate) last_line: RwSignal<usize>,

}

impl Signals {
    pub fn new(cx: Scope, style: &EditorStyle, viewport: Rect, rev: u64, buffer: Buffer, screen_lines: ScreenLines, last_line: usize) -> Self {
        let show_indent_guide =
            cx.create_rw_signal((style.show_indent_guide(), style.indent_guide()));
        let screen_lines_signal = cx.create_rw_signal(screen_lines.clone());
        let viewport = cx.create_rw_signal(viewport);
        let folding_items_signal = cx.create_rw_signal(Vec::new());
        let buffer_rev= cx.create_rw_signal(rev);
        let buffer= cx.create_rw_signal(buffer);
        let last_line= cx.create_rw_signal(last_line);
        Self {
            show_indent_guide,
            viewport,
            folding_items_signal,
            screen_lines_signal, buffer_rev, buffer, last_line
        }
    }
}
