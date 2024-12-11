use floem::reactive::{batch, ReadSignal, RwSignal, Scope, SignalUpdate};
use floem::peniko::Color;
use floem::kurbo::Rect;
use floem::views::editor::EditorStyle;
use crate::lines::buffer::Buffer;
use crate::lines::fold::FoldingDisplayItem;
use crate::lines::screen_lines::ScreenLines;

#[derive(Clone)]
pub struct Signals {
    pub(crate) show_indent_guide: SignalManager<(bool, Color)>,
    pub(crate) viewport: SignalManager<Rect>,
    pub(crate) folding_items: SignalManager<Vec<FoldingDisplayItem>>,
    pub(crate) screen_lines: SignalManager<ScreenLines>,
    pub(crate) buffer_rev: SignalManager<u64>,
    pub(crate) buffer: SignalManager<Buffer>,
    // start from 1
    pub(crate) last_line: SignalManager<usize>,

}

impl Signals {
    pub fn new(cx: Scope, style: &EditorStyle, viewport: Rect, rev: u64, buffer: Buffer, screen_lines: ScreenLines, last_line: usize) -> Self {
        let show_indent_guide =
            SignalManager::new(cx, (style.show_indent_guide(), style.indent_guide()));
        let screen_lines_signal = SignalManager::new(cx, screen_lines.clone());
        let viewport = SignalManager::new(cx, viewport);
        let folding_items_signal = SignalManager::new(cx, Vec::new());
        let buffer_rev= SignalManager::new(cx, rev);
        let buffer= SignalManager::new(cx, buffer);
        let last_line= SignalManager::new(cx, last_line);
        Self {
            show_indent_guide,
            viewport,
            folding_items: folding_items_signal,
            screen_lines: screen_lines_signal, buffer_rev, buffer, last_line
        }
    }

    pub fn trigger(&mut self) {
        batch(|| {
            self.show_indent_guide.trigger();
            self.viewport.trigger();
            self.folding_items.trigger();
            self.screen_lines.trigger();
            self.buffer_rev.trigger();
            self.buffer.trigger();
            self.last_line.trigger();
        });
    }
}

#[derive(Clone)]
pub struct SignalManager<V: Clone + 'static> {
    v: V,
    signal: RwSignal<V>,
    dirty: bool
}

impl <V: Clone + 'static>SignalManager<V> {
    pub fn new(cx: Scope, v: V) -> Self {
        Self {
            signal: cx.create_rw_signal(v.clone()),
            v, dirty: false
        }
    }

    pub fn update_force(&mut self, nv: V) {
        self.v = nv;
        self.dirty = true;
    }
    pub fn trigger(&mut self) {
        if self.dirty {
            self.signal.set(self.v.clone());
            self.dirty = false;
        }
    }

    pub fn signal(&self) -> ReadSignal<V>{
        self.signal.read_only()
    }
    pub fn val(&self) -> &V {
        &self.v
    }
}

impl<V: Clone + PartialEq + 'static> SignalManager<V> {
    pub fn update_if_not_equal(&mut self, nv: V) {
        if self.v != nv {
            self.v = nv;
            self.dirty = true;
        }
    }
}